Chapter 12: Design for Performance
========================================

The parts of the design _and_ performance problem:
1. How does design change to accomodate performance goals?
2. How can evaluate the impact of performance-specific accomodations
   before we have a complete a complete design, much less a program?

First, the design principles that support good design _and_ performance:
1. Minimum information exposed by the executor
  * In practice, take a queue. Callers may need to know if a queue is
    empty (or full), but they do not need to know the size of a queue to know
    these things. Some queue implementations may inherently track size, while
    others may not.
2. Maximum information conveyed by the requestor about intent
  * How much intent can be conveyed is still a function of the executor's
    interface. A caller can't supply a function extra parameters, after
    all.
  * Indexing and iterating a collection are common operations.
    But the intent is very different. A single index tells the executor
    nothing about the next action, but starting an iteration implies the
    caller will want the next element. This intent allows for all kinds
    of optimizations: prefetching, caching, reusable compute, etc.
  * Imagine a tree, where the actual values are stored sequentially in an
    array and the tree nodes contain pointers to this data.
    - Ordered searching will be slower than storing the pointers, as
    they're an extra layer of indirection
    - But _unordered_ searching will be much faster by just reading the
    data array, skipping the pointers completely.

The first point carries a higher risk and is more difficult to change
than the second. As such, give it first consideration.
Weigh known performance goals and breadth of usage before opening the API
to allow more 'intent' to come through. A likely bottleneck used by a
single client will benefit from them being able to express exactly what
they want. A component with no obvious reason to be slow that will have
an unbounded number of clients should be expanded much more
conservatively.

API design for concurrency
----------------------------------------
* Communicate thread-safety guarantees upfront
  * This says nothing about weak vs. strong thread-safety. The author
  mentions preferring weaker at low-level building blocks, and saving
  stronger guarantees for the more visible (transactional) operations.
* For strong thread-safety, add custom locks around the operations that
need it.
  - This may mean guarding _multiple_ ops behind a lock.
    Remember pop'ing a stack: check if empty, read top, remove top.
> The state of the component (class, server, database, and so on) should
> be valid before an API call is made and after it is made
  - But it would then be a waste to give the component ops their own
    locks. This is why low-level pieces should be weakly-safe
    (single-user), allowing for composition within higher-level strongly-safe
    ops.
> The general approach to resolving this contradiction is to do both:
> provide non-locking interfaces that can be used as building blocks of
> higher-level components and provide thread-safe interfaces where it
> makes sense
1. Design a component to have "optional" locking.
  * Check inside would-be transactional ops if we're using locking.
  * You can provide the lock to the component (DI)
  * This can even be coded into the type-signature by making "locking
    policy" classes/structs. They should implement lock ops
    (lock()/unlock()), but the non-locking version will be no-ops and only
    the locking version will have an actual lock.
2. Decorate (wrap) all would-be safe ops with a lock.


Copying and sending data
----------------------------------------
> The reason (data ownership and lifetime management) comes up in the
> context of performance is that often excessive copying is a side effect
> of muddled ownership

The other point made, using an emerging need for data compression,
largely seems to return to the spirit of "protect your API with the
minimum information principle".


Design for optimal data access
----------------------------------------
* If you can't find hot code, look for hot data.
  But how do you find it?
  - Performing all reads through accessor functions lets you count function calls
  and even instrument them to tell you which memory locations.
* When designing, you've got to balance interfaces vs. data layout
  * Making your interfaces "perfect" will likely restrict your data layout
    * Your interface may allow for insertions into an ordered collection
    * But if insertions into the middle of the collection are allowed,
      you've implicitly ruled out storage in an array-like data structure.
    * Or random access vs. iteration
  * And optimal data layouts for some problems will reduce your options
    in implementation but tell your interface what capabilities it can
    reliably expose.
  * A reasonable approach:
    * Alternate between solving for the 'big rocks' of structure and
    interface, spiraling into finer and finer detail.
    * Come up with an efficient storage for the critical-path data.
    * Then design interfaces with that in mind.
* If concurrency is required, think about data sharing.
  * Classify all data as not shared, read-only, or shared for writing.


Performance trade-off
----------------------------------------
### Interface Design
Analyze your read & write needs.
* Reads
  * Random access
  * Streaming
  * Forward iteration
  * Reverse iteration
* Writes
  * Arbitrary insertion
  * Append-only


### Component Design
To me, this section was about the need to reevaluate interface choices at
each component.
An example is given of a Point object vs. a Point collection.
It makes sense to have a Point object, which can be tested in isolation
and reused across the codebase.
But when working with a collection of Points you probably don't want `Collection<Point>`.
You have a set of behaviors in mind - vector operations, transforms, etc.
- that you'll use against a set of points, and that's what the
PointCollection should provide. As a plus, this frees the PointCollection
free to store its Point information however it wants - it may still use
Points at its API, e.g. returning Points from queries and accepting
Points for adding new data, but maybe it unpacks the Point and stores the
info in a 2D array.

### Errors and undefined behavior
Rule #1: Error handling (when nothing goes wrong) must be cheap
Transactions let you rollback, but this can be expensive.
Consider just writing data in a user-inaccessible location and cleaning
up later.

Rule #2: The API defines error communication.

Rule #3: When all else fails, tell the user "this will result in undefined behavior".


Making informed design decisions
----------------------------------------
1. Model
  * Micro-benchmarks are vulnerable to the realities of a real system,
  but it's not that they're lies. The differences they show will still be
  present, it's just that other effects may wash them out.
2. Prototype
  * If you have an existing system that work similarly to what you intend
  to build, try embedding a model of what you plan to do within it.

