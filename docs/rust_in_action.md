# Rust in Action

## 1 Introducing Rust

### 1.1 Where is Rust used?

### 1.2 Advocating for Rust at work

### 1.3 A taste of the language

## 2 Language foundations

### 2.1 Creating a running program

#### 2.1.1 Compiling single files with `rustc`

### 2.10 Making lists of things with arrays, slices, and vectors

#### 2.10.1 Arrays

#### 2.10.2 Slices

Slices are dynamically sized array-like objects.
The term *dynamically sized* means that their size is not known at compile time.
Yet, like arrays, these don't expand or contract.
The use of the word *dynamic* in dynamically sized is closer in meaning to *dynamic typing* rather than movement.
The lack of compile-time knowledge explains the distinction in the type signature between an array (`[T; n ]`) and a slice (`[T]`).

#### 2.10.3 Vectors

Vectors (`Vec<T>`) are growable lists of `T`.
Using vectors is extremely common in Rust code.
These incur a small runtime penalty compared to arrays because of the extra bookkeeping that must be done to enable their size to change over time.
But vectors almost always make up for this with their added flexibility.

### 2.11 Including third-party code

### 2.13 Reading from files

## 3 Compound data types

This chapter focuses on two key building blocks for Rust programmers, `struct` and `enum`.
Both are forms of *compound data types*.
Together, `struct` and `enum` can compose other types to create something more useful than what those other types would be alone.

### 3.1 Using plain functions to experiment with an API

### 3.2 Modeling files with `struct`

We need something to represent that thing we're trying to model.
`struct` allows you to create a composite type made up of other types.
Depending on your programming heritage, you may be more familiar with terms such as object or record.

### 3.3 Adding methods to a `struct` with `impl`

To define methods, Rust programmers use an `impl` block, which is physically distinct in source code from the `struct` and `enum` blocks that you have already encountered.

#### 3.3.1 Simplifying object creation by implementing `new()`

Using `new()` is a convention within the Rust community.
Unlike other languages, `new` is not a keyword and isn't given some sort of blessed status above other methods.

### 3.4 Returning errors

This section discusses different methods for signalling that an error has occurred, beginning with approaches common in other languages and finishing with idiomatic Rust.

#### 3.4.1 Modifying a known global variable

#### 3.4.2 Making use of the `Result` return type

Rust's approach to error handling is to use a type that stands for both the standard case and the error case.
This type is known as `Result`.
`Result` has two states, `Ok` and `Err`.
This two-headed type is versatile and is put to work all through the standard library.

### 3.5 Defining and making use of an `enum`

An *enum*, or enumeration, is a type that can represent multiple known variants.
Classically, an enum represents several predefined known options like the suits of playing cards or planets in the solar system.
The following listing shows one such enum.
```rust
enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
```

### 3.6 Defining common behavior with traits

You have already seen traits in action several times.
Traits have close relatives in other languages.
These are often named interfaces, protocols, type classes, abstract base classes, or, perhaps, contracts.

#### 3.6.1 Creating a `Read` trait

```rust
#[derive(Debug)]
struct File;

trait Read {
    fn read(
        self: &Self,
        save_to: &mut Vec<u8>,
    ) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}
```

#### 3.6.2 Implementing `std::fmt::Display` for your own types

The `println!` macro and a number of others live within a family of macros that all use the same underlying machinery.
The macros `println!`, `print!`, `write!`, `writeln!`, and `format!` all rely on the `Display` and `Debug` traits, and these rely on trait implementations provided by programmers to convert from `{}` to what is printed to the console.

### 3.7 Exposing your types to the world

Your crates will interact with others that you build over time.
You might want to make that process easier for your future self by hiding internal details and documenting what's public.
This section describes some of the tooling available within the language and within cargo to make that process easier.

#### 3.7.1 Protecting private data

Rust defaults to keeping things private.
If you were to create a library with only the code that you have seen so far, importing your crate would provide no extra benefit.
To remedy this, use the `pub` keyword to make things public.

### 3.8 Creating inline documentation for your projects

## 4 Lifetimes, ownership, and borrowing

This chapter explains one of the concepts that trip up most newcomers to Rust - its borrow checker.
The *borrow checker* checks that all access to data is legal, which allows Rust to prevent safety issues.
Learning how this works will, at the very least, speed up your development time by helping you avoid run-ins with the compiler.
More significantly though, learning to work with the borrow checker allows you to build larger software systems with confidence.
It underpins the term *fearless concurrency*.

This chapter will explain how this system operates and help you learn how to comply with it when an error is discovered.
It uses the somewhat lofty example of simulating a satellite constellation to explain the trade-offs relating to different ways to provide shared access to data.
The details of borrow checking are thoroughly explored within the chapter.
However, a few points might be useful for readers wanting to quickly get the gist.
Borrow checking relies on three interrelated concepts - lifetimes, ownership, and borrowing:
* *Ownership is a stretched metaphor*. There is no relationship to property rights. Within Rust, ownership relates to cleaning values when these are no longer needed. For example, when a function returns, the memory holding its local variables needs to be freed. Owners cannot prevent other parts of the program from accessing their values or report data theft to some overarching Rust authority.
* *A value's lifetime is the period when accessing that value is valid behavior*. A function's local variables live until the function returns, while global variables might live for the life of the program.
* *To borrow a value means to access it*. This terminology is somewhat confusing as there is no obligation to return the value to its owner. Its meaning is used to emphasize that while values can have a single owner, it's possible for many parts of the program to share access to those values.

### 4.1 Implement a mock CubeSat ground station

#### 4.1.1 Encountering our first lifetime issue

Movement within Rust code refers to movement of ownership, rather than the movement of data.
*Ownership* is a term used within the Rust community to refer to the compile-time process that checks that every use of a value is valid and that every value is destroyed cleanly.

Every value in Rust is *owned*.

#### 4.1.2 Special behavior of primitive types

Indeed, the only change that we made in listing 4.3 was to wrap our satellite variables in a custom type.
As it happens, primitive types in Rust have special behavior.
These implement the `Copy` trait.

Types implementing `Copy` are duplicated at times that would otherwise be illegal.

### 4.3 What is an owner? Does it have any responsibilities?

In the world of Rust, the notion of *ownership* is rather limited.
An owner cleans up when its values' lifetimes end.

When values go out of scope or their lifetimes end for some other reason, their destructors are called.
A *destructor* is a function that removes traces of the value from the program by deleting references and freeing memory.
You won't find a call to any destructors in most Rust code.
The compiler injects that code itself as part of the process of tracking every value's lifetime.

An implication of this system is that values cannot outlive their owner.
This kind of situation can make data structures built with references, such as trees and graphs, feel slightly bureaucratic.
If the root node of a tree is the owner of the whole tree, it can't be removed without taking ownership into account.

### 4.4 How ownership moves

There are two ways to shift ownership from one variable to another within a Rust program.
The first is by assignment.
The second is by passing data through a function barrier, either as an argument or a return value.

### 4.5 Resolving ownership issues

Rust's ownership system is excellent.
It provides a route to memory safety without needing a garbage collector.
There is a "but," however.

The ownership system can trip you up if you don't understand what's happening.
This is particularly the case when you bring the programming style from your past experience to a new paradigm.
Four general strategies can help with ownership issues:
* Use references where full ownership is not required.
* Duplicate the value.
* Refactor code to reduce the number of long-lived objects.
* Wrap your data in a type designed to assist with movement issues.

#### 4.5.1 Use references where full ownership is not required

The most common change you will make to your code is to reduce the level of access you require.
Instead of requesting ownership, you can use a "borrow" in your function definitions.
For read-only access, use `&T`.
For read-write access, use `&mut T`.

#### 4.5.2 Use fewer long-lived values

If we have a large, long-standing object such as a global variable, it can be somewhat unwieldy to keep this around for every component of your program that needs it.
Rather than using an approach involving long-standing objects, consider making objects that are more discrete and ephemeral.
Ownership issues can sometimes be resolved by considering the design of the overall program.

#### 4.5.3 Duplicate the value

Having a single owner for every object can mean significant up-front planning and/or refactoring of your software.
As we saw in the previous section, it can be quite a lot of work to wriggle out of an early design decision.

One alternative to refactoring is to simply copy values.
Doing this often is typically frowned upon, however, but it can be useful in a pinch.
Primitive types like integers are a good example of that.
Primitive types are cheap for a CPU to duplicate - so cheap, in fact, that Rust always copies these if it would otherwise worry about ownership being moved.

Types can opt into two modes of duplication: cloning and copying.
Each mode is provided by a trait.
Cloning is defined by `std::clone::Clone`, and the copying mode is defined by `std::marker::Copy`.
`Copy` acts implicitly.
Whenever ownership would otherwise be moved to an inner scope, the value is duplicated instead. (The bits of object *a* are replicated to create object *b*.)
`Clone` acts explicitly.
Types that implement `Clone` have a `.clone()` method that is permitted to do whatever it needs to do to create a new value.

#### 4.5.4 Wrap data within specialty types

So far in this chapter, we have discussed Rust's ownership system and ways to navigate the constraints it imposes.
A final strategy that is quite common is to use wrapper types, which allow more flexibility than what is available by default.
These, however, incur costs at runtime to ensure that Rust's safety guarantees are maintained.
Another way to phrase this is that Rust allows programmers to opt in to garbage collection.

To explain the wrapper type strategy, let's introduce a wrapper type: `std:rc::Rc`.
`std:rc::Rc` takes a type parameter `T` and is typically referred to as `Rc<T>`.
`Rc<T>` reads as "R. C. of T" and stands for "a reference-counted value of type `T`."
`Rc<T>` provides *shared ownership* of `T`.
Shared ownership prevents `T` from being removed from memory until every owner is removed.

As indicated by the name, *reference counting* is used to track valid references.
As each reference is created, an internal counter increases by one.
When a reference is dropped, the count decreases by one.
When the count hits zero, `T` is also dropped.

Wrapping `T` involves a calling `Rc::new()`.
