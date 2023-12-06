Chapter 4: Memory Architecture and Performance
========================================

Rust Exercises
----------------------------------------
* Benchmark performance over a growing data set
* Graphing benchmark results
* Comparing sequential writes to a vector vs. list
* Algorithm: The Record Editing Problem
* Algorithm: To Spectre Yourself; reading 'ghost' data with Spectre


Chapter Notes
----------------------------------------
Common CPU freq: 3 GHz
Common RAM freq: 400 MHz

But data rate != memory speed.
To convert, we need to account for Column Access Strobe Latency
(CAS Latency), which roughly means the roundtrip latency to process a
receive, retrieve, and return a read request.

For example, given:
* 3.2 GHz dat rate
* 15 cycle CAS latency
* = 107 MHz memory speed or 9.4 ns/read

The discrepancy between CPU and memory speed gets called "the memory gap."

![The memory hierarchy](./CPU_Memory_Hierarchy.jpg)

The key trade-off between D(ynamic)RAM and S(tatic)RAM is speed for power
consumption. SRAM is faster but uses more power. The L1 cache is the
fastest, but its size is limited to a few KiB by its power usage.

Think in terms of cache sizes. Getting your data set to fit the L1/2/3 cache
greatly eliminates the memory gap as a bottleneck

Things to consider:
1. Does the data fit in a L# CPU cache?
2. What's your access pattern? Sequential or random?
3. What is the size of your atoms? E.g., i64 (8 bytes) vs. char (1-3 bytes)?

> The choice of data structures, or, more generally, data organization,
> is usually the most important decision the programmer makes as far
> as memory performance is concerned

### Optimizing
#### ...the data structure
1. Store data sequentially, e.g. in an array
2. If you don't know the size of the array in advance:
  1. Spend a pass over the data to find out, or
  2. Go with a block-allocated array: a "list" of array blocks, sized to
     fit in the L1/2 cache, that can be grown over time.
3. If you need to arbitrarily insert data, consider copying into a
   node-allocated (list, tree, etc.) data structure for the period of
   time your code has this requirement. Then return to an array-based
   structure.
4. If the data is _occasionally_ accessed in a different order, or will
   be accessed in multiple orders, store the data sequentially and use
   arrays of pointers sorted in the desired order.

> The bottom line is, if we access some data a lot, we should choose a
> data structure that makes that particular access pattern optimal. If
> the access pattern changes in time, the data structure should change as
> well. On the other hand, if we don't spend much time accessing the
> data, the overhead of converting from one arrangement of the data to
> another likely cannot be justified.


**When Profiling**: Look for high cache misses. Best case there is a
single location, but if the root cause is a common data structure used by
many functions, it's up to the observer to piece it together.

#### ...the algorithm
1. Recompute instead of precompute, if retrieving precomputed data
   causes lookups from main memory.
2. Operate in cache-friendly chunks: process what fits in your L# cache
   before loading the next section.
3. Bias towards more fast sequential access in favor of fewer slow, arbitrary access.

### Benchmarking
If the operation being observed is very very quick, the cost of the
benchmarking loop itself may interfere with the measurement. Kind of like
how observing a tiny particle such as an electron can be interfered with
by the photons that let us 'see' it in the first place. Unrolling the
loop manually, e.g. executing the op many times per iteration, can
ameliorate this observational effect.

What can bencharmking results tell us?
* Inflection points after plateaus. Likely means you've exceeded a cache.
* Where memory becomes the bottleneck. At 0.3ns, memory accesses from the
L1 cache can keep the CPU fed. While hitting the L3 cache is 4x slower at
~1.2ns, smart access patterns can make the most of the L1 & L2.
* Whether throughput is yet being effected by payload size. So long as
we're below our bandwidth, we should see latency as the limiting factor.
If latency is increasing/throughput is decreasing as size increases,
   we're hitting a bandwidth limit.

## Hardware Techniques
Prefetch
: Eager retrieval from main memory after detecting sufficient sequential access.
Pipelining
: Parallel execution of non-dependent instructions.

### The Record Editing Problem
```c++
std::list<std::string> data;
// … initialize the records …

for (auto startOfRecord = data.begin(), endOfData = --data.endOfData(), currRecordIdx = startOfRecord
    ; true
    ; startOfRecord = currRecordIdx
) {
    currRecordIdx = startOfRecord;
    ++currRecordIdx;
    const bool done = startOfRecord == endOfData;
    if (must_change(*startOfRecord)) {
        std::string new_str = change(*startOfRecord);
        data.insert(startOfRecord, new_str);
        data.erase(startOfRecord);
    }
    if (done) {
        break;
    }
}
```

### Spectre, in a nutshell
**Summary**: Do an out-of-bounds array index behind a branch that will
never trigger, set to store the data in an array you own. This tricks the
CPU's speculative execution to actually read the memory into cache before
discarding it. Then measure the time it takes to read your array values -
the 'illegal' access will be in cache and much faster than the rest.

Considerations:
* Your storage array must be _big enough_. The example uses an 256
element array of 1024 bytes (chars).
* Your storage array must _not_ be in cache, else you won't be able to
find the 'illegal' data based on lookup speed.
* Your code must never actually execute the out-of-bounds lookup, so you
have to trick the compiler into _thinking_ it will:

    if index < illegal_index { // read memory
* Prefetch would ruin your timings while checking your storage array, so
  you've got to perform them randomly. Even then, noise will require
  you to do this repeatedly.
