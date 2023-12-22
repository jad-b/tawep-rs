Chapter 11: Undefined Behavior and Performance
========================================

First, some definitions of behavior.
__implementation-defined__
: The implementation defines the behavior of all language constructs.
__unspecified__
The implementation is allowed to choose, with some options (maybe) provided by the standard.
__undefined__
: The *entire program* has no requirements on its behavior.

The compiler, in the face of Undefined Behavior (UB), can take a
suprising line of reasoning. Imagine a function with an infinite loop:
* An infinite loop is UB
* The program can only remain well-defined if UB is never executed
* Then this function can never be executed
* So none of the function's callers can be executed
* Maybe the _entire program_ can't be executed

Or maybe it just skips the UB code - any interpretation by the compiler
is valid, because there *are no requirements on the program* in the face
of UB.

Imagine you have a program which manifests an infinite loop _sometimes_.
The compiler can make optimizations, but the 'after' behavior would only
match the 'before' if the loop is not actually infinite. And since
detecting eventual termination is NP-Complete, compiler author's assume
all loops will terminate and proceed with optimizations.

Here's a good one: our string comparison functions in Ch2.
The version using an unsigned int as our loop indexes was _slower_ than
the version using a signed int. UB explains why:
* We were using 32-bit integers on a 64-bit machine
* Signed ints have UB on overflow. Overflowing a 32-bit signed int on a
64-bit machine with the `add` instruction causes it to become a 64-bit integer.
* Unsigned ints wrap around after overflow, returning to zero:
  ...,UINT_MAX-1,UINT_MAX,0,1,...
* To preserve the wraparound behavior of an unsigned 32-bit integer, the
compiler had to replace the fast `add` op with the slower `lea` (load and
extend) op.
* But since overflowing a signed integer is UB, the compiler assumed it
couldn't happen and _left it alone_.

So here, the optimization was to do nothing!
But we also could've switched to 64-bit integers - then `add` would've
had the correct performance on signed _or_ unsigned ints.
Or included the check for the string lengths, allowing the compiler to
determine overflow couldn't occur - but that extra check has its own
cost.

How about this one: using a pointer then including a "if not null" check
in the return expression. Since the pointer was used prior to the return,
the compiler assumes it's not null - else that's UB - and removes your
null check for you.
