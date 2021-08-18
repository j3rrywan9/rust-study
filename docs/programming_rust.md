# Programming Rust, 2nd Edition

## Chapter 2. A Tour of Rust

### rustup and Cargo

The best way to install Rust is to use `rustup`.
Go to [https://rustup.rs](https://rustup.rs) and follow the instructions there.

In any case, once you've completed the installation, you should have three new commands available at your command line:
```
cargo --version

rustc --version

rustdoc --version
```

As a convenience, Cargo can create a new Rust package for us, with some standard metadata arranged appropriately:
```
cargo new hello
```

If our program ever acquires dependencies on other libraries, we can record them in this file, and Cargo will take care of downloading, building, and updating those libraries for us.

Cargo has set up our package for use with the `git` version control system, creating a *.git* metadata subdirectory and a *.gitignore* file.
You can tell Cargo to skip this step by passing `--vcs none` to `cargo new` on the command line.

We can invoke the `cargo run` command from any directory in the package to build and run our program:
```
cargo run
```

When we're through, Cargo can clean up the generated files for us:
```
cargo clean
```

### Rust Functions

The `fn` keyword (pronounced "fun") introduces a function.
Here, we're defining a function named `gcd`, which takes two parameters `n` and `m`, each of which is of type `u64`, an unsigned 64-bit integer.
The `->` token precedes the return type: our function returns a `u64` value.
Four-space indentation is standard Rust style.

By default, once a variable is initialized, its value can't be changed, but placing the `mut` keyword (pronounced "mute," short for *mutable*) before the parameters `n` and `m` allows our function body to assign to them.
In practice, most variables don't get assigned to; the `mut` keyword on those that do can be a helpful hint when reading code.

### Writing and Running Unit Tests

The `#[test]` marker is an example of an attribute.
Attributes are an open-ended system for marking functions and other declarations with extra information, like attributes in C++ and C#, or annotations in Java.
They're used to control compiler warnings and code style checks, include code conditionally (like `#ifdef` in C and C++), tell Rust how to interact with code written in other languages, and so on.
We'll see more examples of attributes as we go.

### Handling Command-Line Arguments

This is a large block of code, so let's take it piece by piece:
```rust
use std::str::FromStr;
use std::env;
```
The first `use` declaration brings the standard library *trait* `FromStr` into scope.
A trait is a collection of methods that types can implement.
Any type that implements the `FromStr` trait has a `from_str` method that tries to parse a value of that type from a string.
The `u64` type implements `FromStr`, and we'll call `u64::from_str` to parse our command-line arguments.
Although we never use the name `FromStr` elsewhere in the program, a trait must be in scope in order to use its methods.

The second `use` declaration brings in the `std::env` module, which provides several useful functions and types for interacting with the execution environment, including the `args` function, which gives us access to the program's command-line arguments.

```rust
let mut numbers = Vec::new();
```
We declare a mutable local variable `numbers` and initialize it to an empty vector.
`Vec` is Rust's growable vector type, analogous to C++'s `std::vector`, a Python list, or a JavaScript array.
Even though vectors are designed to be grown and shrunk dynamically, we must still mark the variable `mut` for Rust to let us push numbers onto the end of it.

We use `Result`'s `expect` method to check the success of our parse.
If the result is an `Err(e)`, `expect` prints a message that includes a description of e and exits the program immediately.
However, if the result is `Ok(v)`, expect simply returns `v` itself, which we are finally able to push onto the end of our vector of numbers.

```rust
let mut d = numbers[0];

for m in &numbers[1..] {
    d = gcd(d, *m);
}
```
This loop uses `d` as its running value, updating it to stay the greatest common divisor of all the numbers we've processed so far.
As before, we must mark `d` as mutable so that we can assign to it in the loop.

So when we iterate, we want to tell Rust that *ownership* of the vector should remain with numbers; we are merely *borrowing* its elements for the loop.
The `&` operator in `&numbers[1..]` borrows a *reference* to the vector's elements from the second onward.
The `for` loop iterates over the referenced elements, letting `m` borrow each element in succession.
The `*` operator in `*m` *dereferences* `m`, yielding the value it refers to; this is the next `u64` we want to pass to `gcd`.
Finally, since `numbers` owns the vector, Rust automatically frees it when `numbers` goes out of scope at the end of `main`.

### Serving Pages to the Web

One of Rust's strengths is the collection of freely available library packages published on the website [crates.io](https://crates.io/).
The `cargo` command makes it easy for your code to use a crates.io package: it will download the right version of the package, build it, and update it as requested.
A Rust package, whether a library or an executable, is called a *crate*; Cargo and crates.io both derive their names from this term.

## Chapter 3. Fundamental Types

To a great extent, the Rust language is designed around its types.
Its support for high-performance code arises from letting developers choose the data representation that best fits the situation, with the right balance between simplicity and cost.

### Tuples

### Pointer Types

### References

### Arrays, Vectors, and Slices

#### Arrays

There are several ways to write array values.
The simplest is to write a series of values within square brackets:
```rust
let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

assert_eq!(lazy_caterer[3], 7);
assert_eq!(taxonomy.len(), 3);
```

#### Vectors

A vector `Vec<T>` is a resizable array of elements of type `T`, allocated on the heap.

A `Vec<T>` consists of three values: a pointer to the heap-allocated buffer for the elements, which is created and owned by the `Vec<T>`; the number of elements that buffer has the capacity to store; and the number it actually contains now (in other words, its length).

#### Slices

A slice, written `[T]` without specifying the length, is a region of an array or vector.
Since a slice can be any length, slices can't be stored directly in variables or passed as function arguments.
Slices are always passed by reference.

A reference to a slice is a *fat pointer*: a two-word value comprising a pointer to the slice's first element, and the number of elements in the slice.

### String Types

#### String Literals

#### Byte Strings

#### Strings in Memory

Rust strings are sequences of Unicode characters, but they are not stored in memory as arrays of `char`s.
Instead, they are stored using UTF-8, a variable-width encoding.
Each ASCII character in a string is stored in one byte.
Other characters take up multiple bytes.

## Chapter 4. Ownership and Moves

### Ownership

In Rust, however, the concept of ownership is built into the language itself and enforced by compile-time checks.
Every value has a single owner that determines its lifetime.
When the owner is freed - *dropped*, in Rust terminology - the owned value is dropped too.
These rules are meant to make it easy for you to find any given value's lifetime simply by inspecting the code, giving you the control over its lifetime that a systems language should provide.

The way to drop a value in Rust is to remove it from the ownership tree somehow: by leaving the scope of a variable, or deleting an element from a vector, or something of that sort.
At that point, Rust ensures the value is properly dropped, along with everything it owns.

That said, the concept of ownership as we've explained it so far is still much too rigid to be useful.
Rust extends this simple idea in several ways:
* You can move values from one owner to another.
This allows you to build, rearrange, and tear down the tree.
* Very simple types like integers, floating-point numbers, and characters are excused from the ownership rules.
These are called `Copy` types.
* The standard library provides the reference-counted pointer types `Rc` and `Arc`, which allow values to have multiple owners, under some restrictions.
* You can "borrow a reference" to a value; references are non-owning pointers, with limited lifetimes.

### Moves

In Rust, for most types, operations like assigning a value to a variable, passing it to a function, or returning it from a function don't copy the value: they *move* it.
The source relinquishes ownership of the value to the destination and becomes uninitialized; the destination now controls the value's lifetime.
Rust programs build up and tear down complex structures one value at a time, one move at a time.

You may be surprised that Rust would change the meaning of such fundamental operations; surely assignment is something that should be pretty well nailed down at this point in history.
However, if you look closely at how different languages have chosen to handle assignment, you'll see that there's actually significant variation from one school to another.
The comparison also makes the meaning and consequences of Rust's choice easier to see.

Consider the consequences of Rust's use of a move here.
Like Python, the assignment is cheap: the program simply moves the three-word header of the vector from one spot to another.
But like C++, ownership is always clear: the program doesn't need reference counting or garbage collection to know when to free the vector elements and string contents.

The price you pay is that you must explicitly ask for copies when you want them.
If you want to end up in the same state as the C++ program, with each variable holding an independent copy of the structure, you must call the vector's `clone` method, which performs a deep copy of the vector and its elements:
```rust
let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
let t = s.clone();
let u = s.clone();
```

### More Operations That Move

### Moves and Control Flow

The previous examples all have very simple control flow; how do moves interact with more complicated code?
The general principle is that, if it's possible for a variable to have had its value moved away and it hasn't definitely been given a new value since, it's considered uninitialized.

### Moves and Indexed Content

### Copy Types: The Exception to Moves

Earlier we were careful to say that *most* types are moved; now we've come to the exceptions, the types Rust designates as `Copy` types.
Assigning a value of a `Copy` type copies the value, rather than moving it.
The source of the assignment remains initialized and usable, with the same value it had before.
Passing `Copy` types to functions and constructors behaves similarly.

## Chapter 5. References

All the pointer types we've seen so far - the simple `Box<T>` heap pointer, and the pointers internal to `String` and `Vec` values - are owning pointers: when the owner is dropped, the referent goes with it.
Rust also has non-owning pointer types called *references*, which have no effect on their referents' lifetimes.

In fact, it's rather the opposite: references must never outlive their referents.
You must make it apparent in your code that no reference can possibly outlive the value it points to.
To emphasize this, Rust refers to creating a reference to some value as *borrowing* the value: what you have borrowed, you must eventually return to its owner.

If you felt a moment of skepticism when reading the phrase "You must make it apparent in your code," you're in excellent company.
The references themselves are nothing special - under the hood, they're just addresses.
But the rules that keep them safe are novel to Rust; outside of research languages, you won't have seen anything like them before.
And although these rules are the part of Rust that requires the most effort to master, the breadth of classic, absolutely everyday bugs they prevent is surprising, and their effect on multithreaded programming is liberating.
This is Rust's radical wager, again.

### References to Values

The right way to handle this is to use references.
A reference lets you access a value without affecting its ownership.
References come in two kinds:
* A *shared reference* lets you read but not modify its referent.
However, you can have as many shared references to a particular value at a time as you like.
The expression `&e` yields a shared reference to `e`'s value; if e has the type `T`, then &e has the type `&T`, pronounced "ref T."
Shared references are `Copy`.

You can think of the distinction between shared and mutable references as a way to enforce a multiple readers or single writer rule at compile time.
In fact, this rule doesn't apply only to references; it covers the borrowed value's owner as well.
As long as there are shared references to a value, not even its owner can modify it; the value is locked down.
Nobody can modify table while show is working with it.
Similarly, if there is a mutable reference to a value, it has exclusive access to the value; you can't use the owner at all, until the mutable reference goes away.
Keeping sharing and mutation fully separate turns out to be essential to memory safety, for reasons we'll go into later in the chapter.

When we pass a value to a function in a way that moves ownership of the value to the function, we say that we have passed it *by value*.
If we instead pass the function a reference to the value, we say that we have passed the value *by reference*.

### Working with References

The preceding example shows a pretty typical use for references: allowing functions to access or manipulate a structure without taking ownership.
But references are more flexible than that, so let's look at some examples to get a more detailed view of what's going on.

### Rust References Versus C++ References

In Rust, references are created explicitly with the `&` operator, and dereferenced explicitly with the `*` operator:
```rust
let x = 10;
let r = &x;
assert!(*r == 10);
```
To create a mutable reference, use the `&mut` operator:
```rust
let mut y = 32;
let m = &mut y;
*m += 32;
assert!(*m == 64);
```
Since references are so widely used in Rust, the `.` operator implicitly dereferences its left operand, if needed:
```rust
struct Anime { name: &'static str, bechdel_pass: bool };
let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
let anime_ref = &aria;
assert_eq!(anime_ref.name, "Aria: The Animation");

// Equivalent to the above, but with the dereference written out:
assert_eq!((*anime_ref).name, "Aria: The Animation");
```
The `.` operator can also implicitly borrow a reference to its left operand, if needed for a method call.
For example, `Vec`'s `sort` method takes a mutable reference to the vector, so these two calls are equivalent:
```rust
let mut v = vec![1973, 1968];
v.sort();           // implicitly borrows a mutable reference to v
(&mut v).sort();    // equivalent, but more verbose
```
In a nutshell, whereas C++ converts implicitly between references and lvalues (that is, expressions referring to locations in memory), with these conversions appearing anywhere they're needed, in Rust you use the `&` and `*` operators to create and follow references, with the exception of the `.` operator, which borrows and dereferences implicitly.

### Assigning References

Assigning a reference to a variable makes that variable point somewhere new:
```rust
let x = 10;
let y = 20;
let mut r = &x;

if b { r = &y; }

assert!(*r == 10 || *r == 20);
```
This behavior may seem too obvious to be worth mentioning: of course `r` now points to `y`, since we stored `&y` in it.
But we point this out because C++ references behave very differently: as shown earlier, assigning a value to a reference in C++ stores the value in its referent.
Once a C++ reference has been initialized, there's no way to make it point at anything else.

### References to References

Rust permits references to references:

### Comparing References

Like the `.` operator, Rust's comparison operators "see through" any number of references:

### References Are Never Null

In Rust, if you need a value that is either a reference to something or not, use the type `Option<&T>`.
At the machine level, Rust represents `None` as a null pointer and `Some(r)`, where r is a `&T` value, as the nonzero address, so `Option<&T>` is just as efficient as a nullable pointer in C or C++, even though it's safer: its type requires you to check whether it's `None` before you can use it.

### References to Slices and Trait Objects

The references we've shown so far are all simple addresses.
However, Rust also includes two kinds of *fat pointers*, two-word values carrying the address of some value, along with some further information necessary to put the value to use.

A reference to a slice is a fat pointer, carrying the starting address of the slice and its length.

Rust's other kind of fat pointer is a *trait object*, a reference to a value that implements a certain trait.
A trait object carries a value's address and a pointer to the trait's implementation appropriate to that value, for invoking the trait's methods.

### Reference Safety

To convey the fundamental ideas, we'll start with the simplest cases, showing how Rust ensures references are used properly within a single function body.
Then we'll look at passing references between functions and storing them in data structures.
This entails giving said functions and data types *lifetime parameters*, which we'll explain.
Finally, we'll present some shortcuts that Rust provides to simplify common usage patterns.
Throughout, we’ll be showing how Rust points out broken code and often suggests solutions.

#### Borrowing a Local Variable

Here's a pretty obvious case.
You can't borrow a reference to a local variable and take it out of the variable's scope:
```rust
{
    let r;
    {
        let x = 1;
        r = &x;
    }
    assert_eq!(*r, 1);  // bad: reads memory `x` used to occupy
}
```
Rust tries to assign each reference type in your program a *lifetime* that meets the constraints imposed by how it is used.
A lifetime is some stretch of your program for which a reference could be safe to use: a statement, an expression, the scope of some variable, or the like.
Lifetimes are entirely figments of Rust's compile-time imagination.
At run time, a reference is nothing but an address; its lifetime is part of its type and has no run-time representation.

In this example, there are three lifetimes whose relationships we need to work out.
The variables `r` and `x` both have a lifetime, extending from the point at which they're initialized until the point that the compiler can prove they are no longer in use.
The third lifetime is that of a reference type: the type of the reference we borrow to `x` and store in `r`.

Here's one constraint that should seem pretty obvious: if you have a variable `x`, then a reference to `x` must not outlive `x` itself, as shown in Figure 5-3.

Here's another kind of constraint: if you store a reference in a variable `r`, the reference's type must be good for the entire lifetime of the variable, from its initialization until its last use, as shown in Figure 5-4.

If the reference can't live at least as long as the variable does, then at some point `r` will be a dangling pointer.
We say that the reference's lifetime must contain or enclose the variable's.

The first kind of constraint limits how large a reference's lifetime can be, while the second kind limits how small it can be.
Rust simply tries to find a lifetime for each reference that satisfies all these constraints.

#### Receiving References as Function Arguments

The signature of `f` as written here is actually shorthand for the following:
```rust
fn f<'a>(p: &'a i32) { ... }
```
Here, the lifetime `'a` (pronounced "tick A") is a lifetime parameter of `f`.
You can read `<'a>` as "for any lifetime 'a" so when we write `fn f<'a>(p: &'a i32)`, we're defining a function that takes a reference to an `i32` with any given lifetime `'a`.

Since we must allow `'a` to be any lifetime, things had better work out if it's the smallest possible lifetime: one just enclosing the call to `f`.
This assignment then becomes a point of contention:
```rust
STASH = p;
```
Since STASH lives for the program's entire execution, the reference type it holds must have a lifetime of the same length; Rust calls this the `'static` lifetime.
But the lifetime of `p`'s reference is some `'a`, which could be anything, as long as it encloses the call to `f`.
So, Rust rejects our code:

At this point, it's clear that our function can't accept just any reference as an argument.
But as Rust points out, it ought to be able to accept a reference that has a `'static` lifetime: storing such a reference in `STASH` can't create a dangling pointer.
And indeed, the following code compiles just fine:
```rust
static mut STASH: &i32 = &10;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}
```

#### Passing References to Functions

#### Returning References

### Omitting Lifetime Parameters

## Chapter 6. Expressions

### An Expression Language

## Chapter 7. Error Handling

Rust's approach to error handling is unusual enough to warrant a short chapter on the topic.
There aren't any difficult ideas here, just ideas that might be new to you.
This chapter covers the two different kinds of error handling in Rust: panic and `Result`s.

### Panic

A program panics when it encounters something so messed up that there must be a bug in the program itself.
Something like:
* Out-of-bounds array access
* Integer division by zero
* Calling `.expect()` on a `Result` that happens to be `Err`
* Assertion failure

### Unwinding

### Aborting

Stack unwinding is the default panic behavior, but there are two circumstances in which Rust does not try to unwind the stack.

### Result

### Catching Errors

This is Rust's equivalent of `try/catch` in other languages.
It's what you use when you want to handle errors head-on, not pass them on to your caller.

`match` is a bit verbose, so `Result<T, E>` offers a variety of methods that are useful in particular common cases.
Each of these methods has a `match` expression in its implementation.
(For the full list of `Result` methods, consult the online documentation. The methods listed here are the ones we use the most.)

### Result Type Aliases

Sometimes you'll see Rust documentation that seems to omit the error type of a `Result`:
```rust
fn remove_file(path: &Path) -> Result<()>
```
This means that a `Result` type alias is being used.

A type alias is a kind of shorthand for type names.
Modules often define a `Result` type alias to avoid having to repeat an error type that's used consistently by almost every function in the module.
For example, the standard library's `std::io` module includes this line of code:
```rust
pub type Result<T> = result::Result<T, Error>;
```
This defines a public type `std::io::Result<T>`.
It's an alias for `Result<T, E>`, but hardcodes `std::io::Error` as the error type.
In practical terms, this means that if you write use `std::io;`, then Rust will understand `io::Result<String>` as shorthand for `Result<String, io::Error>`.

When something like ``Result<()>` appears in the online documentation, you can click on the identifier `Result` to see which type alias is being used and learn the error type.
In practice, it's usually obvious from context.

### Printing Errors

### Propagating Errors

In most places where we try something that could fail, we don't want to catch and handle the error immediately.
It is simply too much code to use a 10-line `match` statement every place where something could go wrong.

Instead, if an error occurs, we usually want to let our caller deal with it.
We want errors to *propagate* up the call stack.

Rust has a `?` operator that does this.
You can add a `?` to any expression that produces a `Result`, such as the result of a function call:
```rust
let weather = get_weather(hometown)?;
```
* On success, it unwraps the `Result` to get the success value inside.
The type of weather here is not `Result<WeatherReport, io::Error>` but simply `WeatherReport`.
* On error, it immediately returns from the enclosing function, passing the error result up the call chain.
To ensure that this works, `?` can only be used on a `Result` in functions that have a `Result` return type.

### Working with Multiple Error Types

### Dealing with Errors That "Can't Happen"

### Ignoring Errors

### Handling Errors in `main()`

### Declaring a Custom Error Type

### Why Results?

## Chapter 8. Crates and Modules

This chapter covers the features of Rust that help keep your program organized: crates and modules.

### Crates

Rust programs are made of *crates*.
Each crate is a complete, cohesive unit: all the source code for a single library or executable, plus any associated tests, examples, tools, configuration, and other junk.

### Editions

To evolve without breaking existing code, Rust uses *editions*.

### Build Profiles

### Modules

### Paths and Imports

The `::` operator is used to access features of a module.
Code anywhere in your project can refer to any standard library feature by writing out its path:
```rust
if s1 > s2 {
    std::mem::swap(&mut s1, &mut s2);
}
```
`std` is the name of the standard library.
The path `std` refers to the top-level module of the standard library.
`std::mem` is a submodule within the standard library, and `std::mem::swap` is a public function in that module.

The alternative is to import features into the modules where they're used:
```rust
use std::mem;

if s1 > s2 {
    mem::swap(&mut s1, &mut s2);
}
```
The `use` declaration causes the name `mem` to be a local alias for `std::mem` throughout the enclosing block or module.

Modules do *not* automatically inherit names from their parent modules.

The keywords `super` and `crate` have a special meaning in paths: `super` refers to the parent module, and `crate` refers to the crate containing the current module.

Using paths relative to the crate root rather than the current module makes it easier to move code around the project, since all the imports won't break if the path of the current module changes.

### Turning a Program into a Library

The first step is to factor your existing project into two parts: a library crate, which contains all the shared code, and an executable, which contains the code that's only needed for your existing command-line program.

### The `src/bin` Directory

### Attributes

Any item in a Rust program can be decorated with *attributes*.
Attributes are Rust's catchall syntax for writing miscellaneous instructions and advice to the compiler.

### Tests and Documentation

As we saw in "Writing and Running Unit Tests", a simple unit testing framework is built into Rust.
Tests are ordinary functions marked with the `#[test]` attribute:
```rust
#[test]
fn math_works() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}
```
Functions marked with `#[test]` are compiled conditionally.
A plain `cargo build` or `cargo build --release` skips the testing code.
But when you run `cargo test`, Cargo builds your program twice: once in the ordinary way and once with your tests and the test harness enabled.
This means your unit tests can live right alongside the code they test, accessing internal implementation details if they need to, and yet there's no run-time cost.

So the convention, when your tests get substantial enough to require support code, is to put them in a `tests` module and declare the whole module to be testing-only using the `#[cfg]` attribute:
```rust
#[cfg(test)]    // include this module only when testing
```

## Chapter 9. Structs

Rust structs, sometimes called *structures*, resemble `struct` types in C and C++, classes in Python, and objects in JavaScript.
A struct assembles several values of assorted types together into a single value so you can deal with them as a unit.
Given a struct, you can read and modify its individual components.
And a struct can have methods associated with it that operate on its components.

Rust has three kinds of struct types, *named-field*, *tuple-like*, and *unit-like*, which differ in how you refer to their components: a named-field struct gives a name to each component, whereas a tuple-like struct identifies them by the order in which they appear.
Unit-like structs have no components at all; these are not common, but more useful than you might think.

### Named-Field Structs

### Tuple-Like Structs

### Unit-Like Structs

### Struct Layout

### Defining Methods with `impl`

You can define methods on your own struct types as well.
Rather than appearing inside the struct definition, as in C++ or Java, Rust methods appear in a separate `impl` block.

Functions defined in an `impl` block are called *associated functions*, since they're associated with a specific type.
The opposite of an associated function is a free function, one that is not defined as an `impl` block's item.

Rust passes a method the value it's being called on as its first argument, which must have the special name `self`.

###

### Type-Associated Functions

An `impl` block for a given type can also define functions that don't take `self` as an argument at all.
These are still associated functions, since they're in an `impl` block, but they're not methods, since they don't take a `self` argument.
To distinguish them from methods, we call them *type-associated functions*.

They're often used to provide constructor functions, like this:
```rust
impl Queue {
    pub fn new() -> Queue {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
}
```

## Chapter 10. Enums and Patterns

### Enums

### Enums with Data

## Chapter 11. Traits and Generics

Of course, this capability is hardly new with Rust.
It's called *polymorphism*, and it was the hot new programming language technology of the 1970s.
By now it's effectively universal.
Rust supports polymorphism with two related features: traits and generics.
These concepts will be familiar to many programmers, but Rust takes a fresh approach inspired by Haskell's typeclasses.

*Traits* are Rust's take on interfaces or abstract base classes.

Code that uses a writer without caring about its type looks like this:
```rust
use std::io::Write;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}
```
The type of `out` is `&mut dyn Write`, meaning "a mutable reference to any value that implements the `Write` trait."

### Using Traits

A trait is a feature that any given type may or may not support.
Most often, a trait represents a capability: something a type can do.
* A value that implements `std::io::Write` can write out bytes.
* A value that implements `std::iter::Iterator` can produce a sequence of values.
* A value that implements `std::clone::Clone` can make clones of itself in memory.
* A value that implements `std::fmt::Debug` can be printed using `println!()` with the `{:?}` format specifier.

Only calls through `&mut dyn Write` incur the overhead of a dynamic dispatch, also known as a virtual method call, which is indicated by the `dyn` keyword in the type.
`dyn Write` is known as a *trait object*; we'll look at the technical details of trait objects, and how they compare to generic functions, in the following sections.

### Trait Objects

```rust
let mut buf: Vec<u8> = vec![];
let writer: &mut dyn Write = &mut buf;  // ok
```
A reference to a trait type, like `writer`, is called a *trait object*.
Like any other reference, a trait object points to some value, it has a lifetime, and it can be either `mut` or shared.

What makes a trait object different is that Rust usually doesn't know the type of the referent at compile time.
So a trait object includes a little extra information about the referent's type.
This is strictly for Rust's own use behind the scenes: when you call `writer.write(data)`, Rust needs the type information to dynamically call the right write method depending on the type of `*writer`.
You can't query the type information directly, and Rust does not support downcasting from the trait object &mut dyn Write back to a concrete type like `Vec<u8>`.

#### Trait object layout

In memory, a trait object is a fat pointer consisting of a pointer to the value, plus a pointer to a table representing that value's type.

In Rust, as in C++, the vtable is generated once, at compile time, and shared by all objects of the same type.

Rust automatically converts ordinary references into trait objects when needed.
This is why we're able to pass `&mut local_file` to `say_hello` in this example:
```rust
let mut local_file = File::create("hello.txt")?;
say_hello(&mut local_file)?;
```

## Chapter 12. Operator Overloading

You can make your own types support arithmetic and other operators, too, just by implementing a few built-in traits.
This is called *operator overloading*, and the effect is much like operator overloading in C++, C#, Python, and Ruby.

### Arithmetic and Bitwise Operators

In Rust, the expression `a + b` is actually shorthand for `a.add(b)`, a call to the `add` method of the standard library's `std::ops::Add` trait.
Rust's standard numeric types all implement `std::ops::Add`.

## Chapter 13. Utility Traits

### `Sized`

A *sized type* is one whose values all have the same size in memory.

All sized types implement the `std::marker::Sized` trait, which has no methods or associated types.
Rust implements it automatically for all types to which it applies; you can't implement it yourself.
The only use for `Sized` is as a bound for type variables: a bound like `T: Sized` requires `T` to be a type whose size is known at compile time.
Traits of this sort are called *marker traits*, because the Rust language itself uses them to mark certain types as having characteristics of interest.

However, Rust also has a few *unsized types* whose values are not all the same size.
For example, the string slice type `str` (note, without an `&`) is unsized.

### `AsRef` and `AsMut`

When a type implements `AsRef<T>`, that means you can borrow a `&T` from it efficiently.
`AsMut` is the analogue for mutable references.

## Chapter 21. Macros

Rust supports *macros*, a way to extend the language in ways that go beyond what you can do with functions alone.

### Macro Basics

`macro_rules!` is the main way to define macros in Rust.

#### Basics of Macro Expansion

Rust expands macros very early during compilation.
The compiler reads your source code from beginning to end, defining and expanding macros as it goes.
You can't call a macro before it is defined, because Rust expands each macro call before it even looks at the rest of the program.

Macro patterns are a mini-language within Rust.
They're essentially regular expressions for matching code.
But where regular expressions operate on characters, patterns operate on *tokens* - the numbers, names, punctuation marks, and so forth that are the building blocks of Rust programs.
This means you can use comments and whitespace freely in macro patterns to make them as readable as possible.
Comments and whitespace aren't tokens, so they don't affect matching.
