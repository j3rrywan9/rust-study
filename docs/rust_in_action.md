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

An implication of this system is that values cannot outlive their owner.

### 4.5 Resolving ownership issues

#### 4.5.1 Use references where full ownership is not required

The most common change you will make to your code is to reduce the level of access you require.
Instead of requesting ownership, you can use a "borrow" in your function definitions.
For read-only access, use `&T`.
For read-write access, use `&mut T`.

#### 4.5.2 Use fewer long-lived values

If we have a large, long-standing object such as a global variable, it can be somewhat unwieldy to keep this around for every component of your program that needs it.
Rather than using an approach involving long-standing objects, consider making objects that are more discrete and ephemeral.
Ownership issues can sometimes be resolved by considering the design of the overall program.
