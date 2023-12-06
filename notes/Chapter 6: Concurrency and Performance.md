Chapter 6: Concurrency and Performance
========================================

> The total CPU time used by all threads is a reasonable proxy for the average power consumption

> On an X86 CPU, there is none (re: perf differences) because the
> hardware instructions for atomic increment and atomic read have the
> "index-like" memory barriers whether we request them or not. On ARM
> CPUs, relaxed (or no-barrier) memory operations are noticeably faster

Rust Exercises
----------------------------------------
* Implement a Spinlock

Calculating Concurrent Performance
----------------------------------------
Will adding concurrency improve performance?
Two questions:
1. Do you have enough work to keep multiple threads busy at all times?
2. Can you sufficiently reduce the use of shared data?

Amdahl's Law

```
s = s_0 / (s_0 * (1-p) + p)
```
Where
* _s_: Total speedup
* _s_0_: Parallelization
* _p_: Fraction you can parallelize

For example, given:
* 256 processors (`s_0 = 256`)
* 1/256 single-threaded (`p = 255/256`)
* Our total speedup (`s`) is:

    s = 256 / (256(1-255/256) + 255/256) = 128.25

Concurrency Control
----------------------------------------
Concurrency and shared data requires synchronization.
Choose from:
### 1) Lock-based/Locks
* Threads wait until they have to lock to proceed
* Types of locks:
  * mutex: Good for long (>=1ms) waits
  * spinlock: Good for very short (<=10ns) waits
  * semaphore

#### Let's make a Spinlock
A Spinlock loops trying to acquire a lock
1=locked, 0=unlocked.

Common inefficiencies:
1. Using CAS when trying to acquire the lock.
  * If it's locked, you're unnecessarily writing 1, locking the cache
  line and causing cache invalidation on the other CPUs waiting for the
  lock.
  * The fix: Read the value first, then try a CAS if it looks unlocked
2. Infinite busy-waiting.
  * The looping looks like useful work from the POV of the OS thread scheduler.
  This can cause the waiting thread to look busier than the thread with
  the lock, starving the thread actually doing work of CPU cycles.
  Note: This _shouldn't_ be a problem if |threads| <= |CPUs|.
  * The fix: Sleep/yield after so many attempts. The number of attempts
  is found through measuring.


#### The Problems with Locks
**The Fundamental Problem with Locks**: They aren't composable.
The semantics of two locks can't be made the same as one.

__Deadlock__
: Given two locks needed to proceed, each of two threads holds the lock the other needs.
__Livelock__
: Given two locks needed to proceed, each of two threads keeps releasing
the lock it has when it can't acquire the other. Kind of like when you're
trying to pass someone in the hall and you keep stepping into each others
way while trying to get out of their way.
__Convoying__
: When the lock holder releases the lock, but due to being 'hot' on the
CPU, can re-acquire the lock before any of the sleeping threads can wake
up.
__Priority Inversion__
: A low-priority thread holds a lock, beating out the high-priority thread.


### 2) Lock-free/Compare and Swap (CAS)
* Will wait, but not lock, other threads if the expected value is changing
* Stores _only_ if an expected value is found
* CAS can be thought of as a primitive atomic operation that can be used to create custom
read-modify-write ops.
* Two variants:
* Strong: Only false if the expected value didn't match
* Weak: Can return false if the expected value matched

How do the problems with Locks lock with Lock-Free?
* Deadlock: Goes away. At least one thread will always succeed,
    guaranteeing progress.
* Livelock: Also goes away. There's no lock to release.
* Convoying: Doesn't exist. All threads are actively trying, so no thread
is better positioned than any other.
* Priority Inversion: Fixed. The high-priority thread, being given more
CPU cycles, should correctly beat a low-priority thread on the CAS op.

#### The Problems with Lock-Free
1. When contention is high, a lot of CPU time is spent retrying the CAS.
2. It's incredibly difficult to reason about. No thread ever stops
   working, memory orders must be considered with respect to all data
   before and/or after the CAS. You have to constantly ask "is there any
   way one thread could see another thread's old data?"

### 3) Wait-free/Atomic variables
* Threads are never explicitly waiting or retrying, as locking happens
at the hardware level of CPU cache lines
* Efficient but limited set of CPU instructions for atomic ops
* Typicaly fastest

Concurrent Data Structures
----------------------------------------
__strong thread safety__
: Can be used concurrently without causing data races or undefined
behavior; "thread-safe".
__weak thread safety__
: Can be read concurrently or read/written by a single thread with
exclusive access; "thread-compatible".

The Publishing Protocol
----------------------------------------
__Problem__
: One thread is creating data, the rest of the threads are reading it -
but never before it's ready.

### Lock-Free Algorithm for Publishing Data
next| https://learning.oreilly.com/library/view/the-art-of/9781800208117/B16229_06_Final_AM_ePub.xhtml#:-:text=The%20lock-free%20solution%20to,%20and%20the%20consumer%20threads
1. Producer writes data to a location it has exclusive access to.
2. Consumers _only_ access data through an atomically-updated
   pointer/index - some kind of address to the actual data.
   This address starts as null/invalid
3. The producer atomically updates the data address, using a release
   memory barrier, ensuring all operations prior to the update (which
   could be dependencies) must be visible as well.
4. Consumers try to read the address with an acquire memory barrier,
   ensuring all following operations (which could have a dependency on
   the producer's data) include the atomic op to read the data.

### Smart pointers for concurrent programming
#### Unique Pointer
* `publish(T*)` atomically stores the new pointer address
* `get()` atomically loads the pointer address
* No thread-safety around construcion.
* No safety for multiple producers

#### Shared pointer
* Counts references to it
* Its internal counter is incremented or decremented atomically
* But: the same shared pointer instance _must not_ be accessed at the
same time. First you make the original shared pointer, then you give each
thread its copy.

Can we do better than the `std::shared_ptr`? Yes, if you limit functionality in optimal ways.
* Intrustive shared pointers store their ref count in the object they
point to. Think a list or tree node. Or use a wrapper class that adds ref
counting.

So how does performance stack up?
1. Unique pointers are the fastest.
2. A custom ref-counted (share) pointer is next.
  * Make it intrusive, and drop all unnecessary functionality.
3. An off-the-shelf ref-counted pointer is last.
