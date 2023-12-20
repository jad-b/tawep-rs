Chapter 9: High-Performance C++
========================================

> designing the architecture and the interfaces that do not lock in poor
> performance and inefficient implementations is the most important
> effort in developing high-performance software.

> Beyond that, the distinction should be made between premature optimization and unnecessary pessimization

Rust Exercises
----------------------------------------

What do you mean, "efficient?"
----------------------------------------
Efficiency isn't a single thing. The following qualities all get used in
regards to a language being efficient:
* Overhead: Only pay for what you use, like those mobile plans for data.
* Optimal: What do you use has been optimized to be fast and/or small.
* Predictable: Use of language features creates expected outcomes from
compilation and runtime.

So what's _inefficient_?

The Trouble With Copying Data
----------------------------------------
* Pass-by-value when pass-by-reference or moving would've sufficed
* At some point, the size of the object being copied will mean its
faster to work via pointer references even with the overhead of pointer
indirection

__return value optimization (RVO)__
: Where the compiler re-uses a memory location for multiple variables of
the same type. Allowed when the variables are connected through lineage,
and no code observes two or more of the variables at the saem time.
However, because the compiler is optimizing a memory copy or move,
user-defined copy or move logic won't be invoked.

Recommendations
* Prefer `move` over `copy`
* Consider 'removing'/'blocking' copy mechanics as the default.
  This will help you discover unintentional copying. If copying is
  required, make it explicit with a dedicated `clone` function.
* Make pass-by-reference or pass-by-pointer (actually a pass-by-value,
  but the value is a pointer) your default. If the value is needed, can
  you move it instead?
* That said, if the value is no bigger than a pointer (probably actually
  a cache line), just copy it (integers and pointers; don't take a
  pointer to a pointer).
* Efficient value return from functions should use RVO and copy elision
as the first-line optimization. Only if those aren't working properly
should you try:
  * Output arguments (?)
  * Factory functions to create values in the heap that get returned with
  smart pointers

Inefficient Memory Management
----------------------------------------
Two core problems:
1. Not enough memory
2. Memory access is too slow

In both cases, reducing memory usage improves things.

### Unnecessary memory allocations
* Optimizing away memory allocations can be a huge speed-up
  - Take a benchmark of writing `0xab` to an N-sized array
  - It is ~2.5x faster to (de)allocate the array once versus on each iteration.
    * This is analogous to using pass-by-value vs. pass-by-reference.
  - It is 1.9x faster to grow the array on demand
    * And the author suspects that tuning the growth policy would bring
    this closer to the one-time preallocation.

### Memory management in concurrent programs
* What are the key problems of memory management?
  * Requesting memory from the OS more than strictly required.
    * Memory allocation from the OS is, by necessity, a global lock.
      The OS allocates to the process, not its threads.
  * Memory being fragmented, wasting it until reclaimed.
  * Just for fun - TIMWOODS from Lean.
    * Transport: NUMA, moving memory between thread arenas
    * Inventory: Holding more memory than is needed
    * Motion: Moving memory
    * Waiting: Waiting for memory allocation
    * Overproduction: Asking for too much memory
    * Over-processing: Oversized tasks (more of a thread than a memory thing)
    * Defects: Can you have bad memory?
    * Skills: Give the threads more control?
* High-performance 'arena' memory allocators abstract this by requesting
  large blocks from the OS - areans- that it parcels out to the program.
  It's a memory allocator for your memory allocations.
  Quartermasters all the way down.
* Thread-local arena allocators, like, `TCMalloc` pull from the global arena as needed (global lock).
  - But they tend to waste memory, with free memory in one thread's
  arena unavailable to a neighboring thread
  - Cross-thread deallocation - where one thread is deallocating memory
  alloc'd by another, such as when deleting a node from a tree - is
  particularly slow. The memory must be transferred out of the owner's
  arena and into the deletor's or shared arena.
  - Just using another thread's memory can range in performance, thanks
  to NUMA

__non-uniform memory architecture (NUMA)__
: The reality that, on hardware, memory banks are not equally close to all CPUs.


#### Block Allocators
Block allocators do a good job at controlling fragmentation and reducing OS allocations
* The primary allocators requests "large" (like 8MiB, which fits in a L3 cache) blocks of memory from the OS,
  and parcels out smaller blocks, like 64KiB
* A secondary allocator can receive the 64KiB blocks and parcel them into even smaller blocks
  * These secondary allocations don't have to be uniform in size - a
  64b/8B integer here, a 256B string there - but staying uniform reduces
  the overhead of tracking how much is space is left in the block
* A third allocator can etc., etc.
* How do blocks reduce waste?
  * It caps waste to (size of block) - (smallest possible allocation)
  * Blocks are easily reused. A returned block is ready for the next
  allocation request (FIFO).
  * And a FIFO cache has a benefit of reusing "hot" blocks already in a CPU cache.
* Thread-local block buffers can be layered on as well.

Block allocators do influence your use of data structures
* You can't use a single vector, as that expects a contiguous allocation of memory.
* But a deque - an array-like backed by blocks - works great
* Certain patterns work better than others too
  * Many small DS, each given an allocation, will waste most of their block
  * More efficient is to have a larger DS that oversees the
  packing of smaller DS into the block.

### Optimization of conditional execution
Number three on the "top wastes" - poor pipelining, led by conditional operations.

Some calibration:
- Good: 0.1% mispredicated branches
- Bad: >=1% mispredicted branches

Finding what branches are hard to predict:
- Short-circuiting boolean operators (`||` and `&&`) can be suspect
  * Once, and only once, you've proven these are at fault should you
  switch to boolean math:
    * `a||b||c == a + b + c`, so long as `0|1 = false|true`
    * `(if b then s += x) == (s += b * x)`

Guess what's almost never worth optimizing?
* Ternary return expressions! `return x if x > 0 else 0`
  * Typically handled by a `CMOVE` instruction, which doesn't require speculative exceuction
    and lead to a pipeline flush.
* Function calls behind a conditional
  * Optimization - through the trick of replacing the conditional with an
    array of function pointers indexed by the condition expression - can
    disrupt inlining. And function inlining is of bigger benefit than branch prediction
  * And even if they weren't inlined, the extra indirection at runtime to
    lookup the function can be as big a hit as branch misprediction.
  * The real answer here is to rewrite the code such that this
    conditional function execution isn't happening in the hot path.

