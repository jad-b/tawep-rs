Chapter 5: Threads, Memory, and Concurrency
========================================

Rust Exercises
----------------------------------------
* benchmark
  - multi-threaded i64 writes
  - performance of no sharing, false sharing, atomic, mutex


Chapter Notes
----------------------------------------
waiting on memory != waiting on i/o
memory waits increase time per instruction. the cpu literally can not
process an instruction until the register is loaded
i/o waits become an OS syscall

symmetric multi-threading (SMT):
same single set of registers and computing units
double the program counters and thread-saving hardware

> the performance of a concurrent program largely depends on how
> independently the threads can work.
> This is determined, first and foremost, by the algorithm and the
> partitioning of work between threads

Accesses to shared data wil __never scale__.
All performance improvement from multi-threading comes from independent computation.

### A Quick Note on Data Size
* CPUs _operate_ from 1 byte -> a 'word', which depends on the size of the
variable. A `unsigned long` is an 8-byte word.
* But data _moves_ in chunks from memory to cache. The minimum sized
'chunk' is a cache line. All x86 CPUs have a 64 byte cache line.
* This means the smallest lock a CPU can take on memory in practice is
the size of a cache line.
* If two threads are atomically writing to data closer than a cache line, one thread
will be locked out while the other works.
* And even unguarded data can be affected by cache line length. The
Disruptor saw that unmodified sequence counters were getting
unnecessarily invalidated, becase a sequence counter that was modified
sat in the same cache line. This is termed 'false sharing'. Their
solution was to artificially pad the counters to take up the entire cache
line (64 bytes), so only legitimate writes would mark it as invalid. Of
course, it was still vulnerable to eviction due to a full cache, but
that's a different discussion.

### Memory Barriers
* Memory barriers are guarantees around the visibility of atomic
operations across CPUs.
* An acquire-release memory barrier ensures:
  * Ops before/after an barrier _only_ become visible before/after the
  barrier is visible
  * This lets one atomic variable checkpoint other data accesses, like
  guarding a queue.
* A release memory barrier ensures:
  * All ops before an barrier become visible before the atomic op does
  * But ops _after_ the barrier may be visible before the atomic op
  * 'at least or more past the barrier', i.e. `>=`
* An acquire memory barrier ensures:
  * All ops after the barrier only become visible after the atomic op
  * But previous ops may also become visible after the barrier
  * 'up to the barrier', i.e. `<=`

Factors of multi-threaded performance
----------------------------------------
* Shared L# cache(s)
* NUMA: Some memory is closer to some CPUs
* Main memory bandwidth, shared across CPUs
  May show as increasing threads having no or negative impact.
  Plot: (Data size vs. throughput (words/ns), ranging over thread #)
  Or plot: (thread # vs. thoughput (relative to 1 thread), ranging over data size)

Optimizing
----------------------------------------
* Data sets need to fit into non-shared L1/2 caches
* The trade-off of fewer memory access for more computation tilts further
to the latter
* sequential acces << random access

__False Sharing__
: Threads operating on independent data can have a hidden dependency on
data 'transport' if their ranges share a cache line, i.e. 'false sharing'.

next| Memory order in C++
