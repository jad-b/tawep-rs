Chapter 4: Memory Architecture and Performance
========================================

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
__Prefetch__:
 Eager retrieval from main memory after detecting sufficient sequential access.

next|
> Another performance optimization technique that the hardware employs very successfully is the familiar one
