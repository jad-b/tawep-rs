Chapter 7: Data Structures for Concurrency
========================================

> In general, you should never design a lock-free data structure until
> you know that you need one: developing lock-free code may be cool, but
> trying to find bugs in it is most definitely not

> As we have seen many times, when it comes to designing concurrent data
> structures, unnecessary generality is your enemy. Build only what you
> need.

Rust Exercises
----------------------------------------
* Double-checked locking

Implementing Data Structures: The Stack
----------------------------------------
Identify what operations need concurrency controls by what needs to be
_transactional_.
* A thread-unsafe Stack can check if its empty, peek at the top, and pop
a value as three separate actions.
* But a thread-safe Stack must guard all those actions behind the same
lock, else state can change from the time you've checked size or peeked.

### Optimizations
* Since locking is slow, allowing batching your writes (`pop` or `push`)
  - only useful if locking is slower than the critical section
* Use a read-write lock: Allows many simultaneous readers, or a single
  writer.
  - is slower than a normal mutex
* Take advantage of _your_ application's specific use of the data
structure.
  * Data is `push`'d by a single writer before being consumed by many
  readers? Remove the lock for `push`.
  * Data is added and read multi-threaded, but never written _and_ read at
  the same time? A wait-free atomic index to track the top of the array
  will be much faster than a lock (although see note about hardware).
* Lock-based vs. Lock-free vs. Wait-free - what's fastest?
  * Depends on hardware.
  * ARM systems tend to have higher cores with slower processors.
    And CAS tends to outperform atomic instructions for them.

#### Going Lock-free (CAS)
This will allow for concurrent producers _and_ consumers, with the
penalty of a mutex.

##### Algorithm: Conditional Atomic Increment (or Decrement)
1. Read current value
2. Until successful:
  1. Check bounds: if final value would be too low or high -> error
  2. Try a weak CAS
    * _weak CAS_: Allowed to falsely report current != expected
3. **Optimization**: Yield after some (~8-16) unsuccessful attempts.
  * A single nanosecond sleep on Linux is sufficient

##### Algorithm: Tracking free & ready
**Pushing**
```
free, ready' = cas_weak(exp=(free, ready)
                       ,new=(free, ready + 1)
                       ,acquire
                       ,relaxed)
data[ready] = <data>
free', ready' = cas_strong(exp=(free, ready')
                          ,new=(free+1, ready')
                          ,release
                          ,relaxed)
```
**Poppin'**
```
free' = atomic(free - 1)
result = &data[free]
ready' = atomic(ready - 1)
return result
```

> But how do I atomically update _two_ integers?
Make two 32-bit integers fit in a 64-bit word using a zero-cost struct or bit
manipulating them into a 64-bit long.

Implementing Data Structures: The Queue
----------------------------------------
* Producers and consumers do not compete unless the queue is empty
  - Unlike a Stack, where both operate on the top of the stack
  - Suggestion: When Ps & Cs work in different areas of the data structure,
    start with the scenario where these are different threads
* Memory layout: Array
  - Back index for producers
  - Front index for consumers
  - Shared data:
    + Atomic count of size
* Operations:
  - `push(x)`
  - `pop -> Optional<x>`
* Mult-producer multi-consumer (MPMC)
  - Can no longer be wait-free - the queue size can change after
  checking, leading to illegal behavior.
  - Can CAS both front & back indices atomically by packing them into a
  single 64-bit atomic word.
  - Shared data:
    + Queue size
    + Front & back indices

### Optimizations
* If the queue is full, have the producer moonlight as a consumer and
process the data it would've enqueued until the queue has space.
* What if we give up sequential consistency?
  * Divide the entire queue into sub-queues
  * Guard access to these sub-queues with atomic pointers, acquired by
  CAS with a null pointer (null == busy)
  * Once locked, the thread can write without concurrency guards (no contention)
  * Shared data:
    - Atomic pointers to sub-queues (lock-free CAS)
    - Total queue size (wait-free atomic +/-)
* What's the cost?
  - Queue can falsely report empty
  - Output of elements will _not_ be totally ordered, but kinda ordered _on average_
* When is this a good idea then?
  - You need a fast way to shovel data from Ps to Cs, and there are no
  data dependencys between the elements.
  - Consider an HTTP server
    - Incoming requests can only have dependencies on a response, meaning
    its parent data has been removed from the queue.


_sequential consistency_
: A guarantee that:
1. The same output is produced in concurrent or serial execution
2. The same order of operations by any given actor is the same across
   concurrent or serial execution

Memory management for concurrent data structures
----------------------------------------
> The lock-guarded and the non-sequentially-consistent data structures we have
> seen ... do not have this problem (memory allocation): under the lock or
> exclusive ownership, there is only one thread operating on the particular data
> structure, so the memory is allocated in the usual way.

Allocating additional memory (and presumably deallocating?) under
concurrent access is a specific challenge for lock-free (CAS)
implementations.
* But we mustn't confuse concurrent data structures for thread-safe data
structures
* If our DS uses exclusive ownership - lock or CAS guarded pointer acquisition -
  then we allocating more memory is already protected behind a
  thread-safe mechanism.
* But when lock-free/using CAS, competing threads never stop trying their
operation. This can easily lead to some very weird behavior.

Solutions:
* Best option: Avoid memory allocation (during concurrent access).
  1. Preallocate the maximum amount of possible memory
  2. Backpressure the caller
* If you must add memory, avoid copying the entire data structure.
  * Compose your data structure of fixed-size memory blocks, allowing you
  to (de)allocate blocks instead of everything.
* If you have to frequently add memory:
  * **You probably shouldn't be using a concurrent data structure**
  * A single-threaded data structure using a exclusive ownership
  (lock/CAS-guarded pointer) probably outperforms a lot of memory
  allocations.
* Performance during memory allocation isn't critical. The important
performance is when we're _not_ allocating more memory.

### Algorithm for thread-safe memory allocation
summary: Guard infrequent but problematic events behind a global lock
while minimizing its impact during normal operation.

__double-checked locking__
: Using an atomic flag to signal an activity is in-progress, and a lock for execution of the
activity.

```
Atomic<bool> waiting = false;
Mutex lock = ;

while waiting {}; // Busy wait

if ( // Out-of-memory)
  lock.acquire();

  // Gotta check our OOM condition _again_ after getting the lock
  // Race conditions, man.
  if ( // still out-of-memory)
    waiting = true;
    // allocate memory
    waiting = false;

  lock.release();

// normal operation...
```

Implementing Data Structures: The List (and Other Node-Based DS's)
----------------------------------------
__nodal data structure__
: Where individual elements are stored in ocations called "nodes",
with their order maintained by pointers. The memory layout is permitted,
and likely, to be non-contiguous.

* Operations:
  - `pop_front()`: Should work whether empty or not
  - `push_front(x)`
  - `insert_after(x)`
* Memory layout: Array
  + Shared Data:


### Considerations
* Almost certainly requires the use of reference-counted smart pointers.
  * (jdb) The book specifically calls out this problem with list
  iterators. Possible that RC'd pointers aren't required if iteration
  isn't needed (would only allow for O(1) reads).
* Global locks are only viable when your program accesses the nodal DS
against through its "edges" - head, tail, root, leaf nodes - as these
operations don't require locking multiple nodes.
  * Node-level locking is susceptible to both deadlocks _and_ livelocks
* A performant concurrent nodal DS will be lock-free
  * But this is very complex, so here are some alternatives to consider
    first:
    - Copy the data you need into a thread-local DS
    - Divide the DS into single-threaded partitions

### Lock-Free List
next| Lock-Free List

### Optimizations
* A lightweight memory allocator can reduce total allocations. It can
serve as a buffer for memory, receiving allocations in chunks but
distributing node-sized fragments, and returning "deallocated" nodes back
to its pool without releasing it to the OS.

