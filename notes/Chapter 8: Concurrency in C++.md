Chapter 8: Concurrency in C++
========================================

(jdb| These notes are much lighter than previous chapters, as my focus
was only on being able to understand C++ at the time, but not write it.
As such, they were taken with a bent towards what to look for in other
languages, such as Rust.)

Rust Exercises
----------------------------------------
* Document Rust's `async` model
  * Is it stackless or stateful?
* Code: Passing an async function between threads

The C++ Memory Model
----------------------------------------
A specification of "how threads interact through memory."

* C++11 saw us
  * Define "What is a thread?"
    + jdb| Do they mean the memory layout of a thread, its API, or...?
  * Synchronization primitives
    * Mutex, lock(), condition_variable - largely followed POSIX features
* C++17 brought
 * scoped_lock
 * shared_mutex
 * L1 cache line size determination (hardware_destructive_interference_size)
 * **Parallel algorithms**
* C++20
  * **coroutines**
  * This is interesting.

Coroutines
----------------------------------------
__stackful__
: Maintain stack on the stack. Also called _fibers_. More flexible, in
that you can suspend execution at any point, no matter how deep in the
call stack from the coroutine you are.

__stackless__
: Maintain state on the heap. Can _only_ be suspended from the top-level
of the coroutine, but is much more memory & CPU efficient.

C++ coroutines are...complex.
Some points of interest:
* In short, implementing a C++ coroutine w/o help from a library
requires:
  1. A function that uses `co_yield <value>` or `co_await
     awaitable{<value>}` and returns a special type that...
  2. Has a special nested type `promise_type` (named exactly such)
     that...
  3. Defines Promise methods
* Values are returned from coroutine executing by passing them "through"
the promise - that is, the value is stored in the promise type for
retrieval by the coroutine's caller. Callers invoke `co_yield <val>`,
which fires the `yield_value(<val>)` Promise method.
*
* The Promise type requires method definitions for construction, first
run, last run, unhandled exception, and how to yield values.
* Most of implementing a coroutine goes into its Promise type.
* What's unclear:
  - Can arguments be passed to the Promise type? Would you even want to,
  since the top-level coroutine is a function that accepts parameters?

When to use coroutines?
* Work stealing - easily transfer tasks between workers (threads)
* Lazy generators
* I/O
* Event handling

Could _you_ have implemented coroutines (with threads?)
* Operationally, coroutines require a stop/start button. You could
probably mimic this with a thread class that accepted such a handle.
* Memory-wise, no. At least not stackless coroutines. Threads have an
associated stack frame, and while they can make use of heap memory, it
wouldn't be as lightweight.

Are coroutines isomorphic to an event processor?
* Coroutines: Discrete, segmented bodies of work
* Events: Discrete facts. Events often cause work.
* What is the difference between suspending a coroutine and emitting an event?
