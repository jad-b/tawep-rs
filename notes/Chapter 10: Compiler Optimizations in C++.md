Chapter 10: Compiler Optimizations in C++
========================================

jdb) As with the chapter 8, I skim over the depths of the explanations
specific to C++, given I'm not writing it at the moment.

Rust Exercises
----------------------------------------
* How can you tell if a function was inlined?

It's What You Can Prove
----------------------------------------
Consider a dumb-simple loop:
```
vector<int> v;
for (int x in v) { ++x; }
```
Unrolling the loop - executing multiple work instructions per iteration -
will improve performance, because there's a hidden `if not at end of
data` conditional check being performed at every end.
We could do this manually - `++x; ++x; ++x; ...` - but the compiler can
do it automatically _if_ it can prove it won't exceed the bounds of the
loop. A fixed-size array is provably-bounded - `int[16] v;` - but a
vector defined using a constant still _isn't_. What if vector gets
resized? The compiler could only hope to know this if all code accessing
the vector lives within the loop, explicitly or through inlining.

This example hits on three key topics:
1. Non-inlined functions can't be optimized like inlined functions.
   They compiler _must_ assume could it can't see can do anything.
2. Global and shared variables prevent a lot of optimization.
3. Short and simple gets more readily optimized than long and complex.

Function Inlining
----------------------------------------
__inlining__
: Replacing a function call with the function code itself.

* Most compilers optimize at a level smaller than the entire program
* They're size-restricted by how much code they can consider before
hitting a complexity asymptote.
* Inlining lets the compiler see more code in the path
  - Two requirements:
    1. Function code is visible during compilation of the caller
    2. Function being called is known during compilation
      - Rules out virtual function calls and calls through function pointers
* The benefits of inlining
  1. Removes the function call itself (looking up the function and
     loading its data and instructions)
  2. Lets the compiler rule out a whole bunch of possibilities it must
     account for, thus leaving the code untouched
    * Side effects - can't remove or reduce calls
    * Order of execution - like around mutexes and memory barriers
    * Local state - can't remove invocations, in case each one changes state (think RNGs)
    * References - they're _everybody's_ girl

> This is the key to inlining and its effect on optimization: inlining
> allows the compiler to see what is not happening inside the otherwise
> mysterious function

### Another benefit of inlining: Customization
* While inlining, the compiler can customize the function for its specific usage
  * This is very similar to _monomorphization_, where the compiler
  generates a type-specific instance for every concrete use of a generic type.
    * Given: `std::find_if(v.begin(), v.end(), [&](int i) { return pred(i); })`
    * Was: `(InputIt first, InputIt last, UnaryPredicate p );`
    * Is: `(InputIt first, InputIt last, bool (*)(int) p );`
* *Note:* Function parameters can typically only be inlined if the outer function is inlined as well
  * But providing a _lambda expression_ encourages inlining, oddly enough
    - All lambdas have unique types, helping the compiler reify the generics as there's only a single match
    - Being unique they're only called once, so inlining is of equivalent cost

### The Cost of Virtual Functions
__virtual function__
: A function whose identity is deferred until runtime. Enables dynamic
dispatch and overriding in OOP inheritance.

* If the C++ compiler can guarantee the runtime type, it can convert
virtual functions to non-virtual in what's called _devirtualization_.
* And this matters, because _you can't inline a virtual function_.

What does the compiler really know?
----------------------------------------
A few points of interest in here:
* Declaring redundant local variables can help the compiler see that the
  data is not changing. It may optimize them out of your binary, but still
  make use of the information during compilation.
* Reference parameters of the same type means the compiler must consider
  they're actually the same data.
  * Again, a local variable can inform the compiler this isn't the case.
* What's probably not worth caring about? Removing if-checks guarding
  function arguments. More important is to keep the functions inline-able,
  in which case the compiler will remove redundancy for you.

Lifting knowledge from runtime to compile time
----------------------------------------
Adding type information through the use of generics (C++ says
"templates") can help the compiler make decisions it'd otherwise leave to
runtime.

Here the author shows an interesting technique that I can't quite tell is
specific to C++. He moves a reference parameter, used in a conditional
check guarding a choice of function call, a template parameter that
is still referenceable by the function. No equivalent in Java or Rust
come to mind - accessing type information at runtime - at least without
reflection or macros.

Using Java as an example, I think the closest is duplicating the
information into a generic type paramter. Turning this:

    void process(List<Shape> v, OpType op);

into this:

    void <T extends OpType> process(List<Shape> v, T op);

Now the compiler can see `process<ToShrink>(shapes)`, inline the exact
code and remove any provably-unnecessary checks.

### Advanced: Generically-typed Lookup Tables
Say you've got a hot-path computation that takes different forms
depending on the inputs, leaving you with many slightly-different
implementations. The example used is needing to calculate a specific set
of properties on millions of graphics objects.

If you encode the selection of properties into the type signature itself
- `<bool need_length, bool need_width, ...>` - then you can have a
top-level function that dispatches to the correct implementation -
`<true, false, true, ...>my_func(...)`. The compiler will optimize away
the actual lookup during compilation.

(jdb| This is all very reminiscent of embedding state machines in the
 type signatures of functions)
