# Rust for Rustaceans

## Chapter 1: Foundations

### Talking About Memory

Not all memory is created equal.
In most programming environments, your programs have access to a stack, a heap, registers, text segments, memory-mapped registers, memory-mapped files, and perhaps nonvolatile RAM.
Which one you choose to use in a particular situation has implications for what you can store there, how long it remains accessible, and what mechanisms you use to access it.
The exact details of these memory regions vary between platforms and are beyond the scope of this book, but some are so important to how you reason about Rust code that they are worth covering here.

#### Memory Terminology

#### Variables in Depth



### Borrowing and Lifetimes

#### Shared References

#### Mutable References

#### Interior Mutability

#### Lifetimes

A *lifetime* is really a name for a region of code that some reference must be valid for.

## Chapter 2: Types

### Types in Memory

Every Rust value has a type.
Types serve many purposes in Rust, as we'll see in this chapter, but one of their most fundamental roles is to tell you how to interpret bits of memory.
When you define your own types, it's the compiler's job to determine where each part of the defined type goes in the in-memory representation for that type.

#### Alignment

For this reason, all values, no matter their type, must start at a byte boundary.
We say that all values must be at least *byte-aligned* - they must be placed at an address that is a multiple of 8 bits.

#### Layout

Now that you know about alignment, we can explore how the compiler decides on the in-memory representation, known as the *layout*, of a type.

#### Complex Types

#### Dynamically Sized Types and Wide Pointers

You may have come across the marker trait `Sized` in various odd corners of the Rust documentation and in error messages.
Usually, it comes up because the compiler wants you to provide a type that is `Sized`, but you (apparently) did not.
Most types in Rust implement `Sized` automatically - that is, they have a size that's known at compile time - but two common types do not: trait objects and slices.
If you have, for example, a `dyn Iterator` or a `[u8]`, those do not have a well-defined size.
Their size depends on some information that is known only when the program runs and not at compile time, which is why they are called *dynamically sized types (DSTs)*.
Nobody knows ahead of time whether the `dyn Iterator` your function received is this 200-byte struct or that 8-byte struct.
This presents a problem: often the compiler must know the size of something in order to produce valid code, such as how much space to allocate to a tuple of type `(i32, dyn Iterator, [u8], i32)` or what offset to use if your code tries to access the fourth field.
But if the type isn't `Sized`, that information isn't available.

The compiler requires types to be `Sized` nearly everywhere.
Struct fields, function arguments, return values, variable types, and array types must all be `Sized`.
This restriction is so common that every single type bound you write includes `T: Sized` unless you explicitly opt out of it with `T: ?Sized` (the `?` means "may not be").
But this is pretty unhelpful if you have a DST and want to do something with it, like if you really want your function to accept a trait object or a slice as an argument.

The way to bridge this gap between unsized and sized types is to place unsized types behind a *wide pointer* (also known as a *fat pointer*).
A wide pointer is just like a normal pointer, but it includes an extra word-sized field that gives the additional information about that pointer that the compiler needs to generate reasonable code for working with the pointer.
When you take a reference to a DST, the compiler automatically constructs a wide pointer for you.
For a slice, the extra information is simply the length of the slice.
For a trait object - well, we'll get to that later.
And crucially, that wide pointer is `Sized`.
Specifically, it is twice the size of a `usize` (the size of a word on the target platform): one `usize` for holding the pointer, and one `usize` for holding the extra information needed to “complete” the type.

### Traits and Trait Bounds

Traits are a key piece of Rust's type system - they are the glue that allows types to interoperate even though they don't know about each other at the time they are defined.

#### Compilation and Dispatch

When you write a type or function that is generic over `T`, you're really telling the compiler to make a copy of that type or function for each type `T`.
When you construct a `Vec<i32>` or a `HashMap<String, bool>`, the compiler essentially copy-pastes the generic type and all its implementation blocks and replaces all instances of each generic parameter with the concrete type you provided.
It makes a full copy of the `Vec` type with every `T` replaced with `i32`, and a full copy of the `HashMap` type with every `K` replaced with `String` and every `V` with `bool`.

The same thing applies to generic functions.
```rust
impl String {
  pub fn contains(&self, p: impl Pattern) -> bool {
    p.is_contained_in(self)
  }
}
```
A copy of this method is made for every distinct pattern type (recall that `impl Trait` is shorthand for `<T: Trait>`).
We need a different copy of the function body for each `impl Pattern` type because we need to know the address of the `is_contained_in` function to call it.
The CPU needs to be told where to jump to and continue execution.
For any *given* pattern, the compiler knows that that address is the address of the place where that pattern type implements that trait method.
But there is no one address we could use for *any* type, so we need to have one copy for each type, each with its own address to jump to.
This is referred to as *static dispatch*, since for any given copy of the method, the address we are "dispatching to" is known statically.

This process of going from a generic type to many non-generic types is called *monomorphization*, and it's part of the reason generic Rust code usually performs just as well as non-generic code.
By the time the compiler starts optimizing your code, it's as if no generics were there at all!
Each instance is optimized separately and with all of the types known.

Monomorphization also comes at a cost: all those instantiations of your type need to be compiled separately, which can increase compile time if the compiler cannot optimize them away.
Each monomorphized function also results in its own chunk of machine code, which can make your program larger.
And because instructions aren't shared between different instantiations of a generic type's methods, the CPU's instruction cache is also less effective as it now needs to hold multiple copies of effectively the same instructions.

The alternative to static dispatch is *dynamic dispatch*, which enables code to call a trait method on a generic type without knowing what that type is.
In practice, the caller gives us a pointer to a chunk of memory called a virtual method table, or *vtable*, that holds the address of the implementation of all the trait's methods for the type in question,

You'll notice that when we opted in to dynamic dispatch using the `dyn` keyword, we had to place an `&` in front of it.
The reason is that we no longer know at compile time the size of the pattern type that the caller passes in, so we don't know how much space to set aside for it.
In other words, `dyn Trait` is `!Sized`, where the `!` means not.
To make it `Sized` so we can take it as an argument, we place it behind a pointer (which we know the size of).
Since we also need to pass along the table of method addresses, this pointer becomes a wide pointer, where the extra word holds the pointer to the vtable.
You can use any type that is able to hold a wide pointer for dynamic dispatch, such as `&mut`, `Box`, and `Arc`.
```rust
impl String {
  pub fn contains(&self, p: &dyn Pattern) -> bool {
    p.is_contained_in(&*self)
  }
}
```
The combination of a type that implements a trait and its vtable is known as a *trait object*.
Most traits can be turned into trait objects, but not all.

Dynamic dispatch cuts compile times, since it's no longer necessary to compile multiple copies of types and methods, and it can improve the efficiency of your CPU instruction cache.
However, it also prevents the compiler from optimizing for the specific types that are used.
With dynamic dispatch, all the compiler can do for `contains` in Listing 2-2 is insert a call to the function through the vtable - it can no longer perform any additional optimizations as it does not know what code will sit on the other side of that function call.
Furthermore, every method call on a trait object requires a lookup in the vtable, which adds a small amount of overhead over calling the method directly.

When you're given the choice between static and dynamic dispatch, there is rarely a clear-cut right answer.
Broadly speaking, though, you'll want to use static dispatch in your libraries and dynamic dispatch in your binaries.
In a library, you want to allow your users to decide what kind of dispatch is best for them, since you don't know what their needs are.
If you use dynamic dispatch, they're forced to do the same, whereas if you use static dispatch, they can choose whether to use dynamic dispatch or not.
In a binary, on the other hand, you're writing the final code, so there are no needs to consider except those of the code you are writing.
Dynamic dispatch often allows you to write cleaner code that leaves out generic parameters and will compile more quickly, all at a (usually) marginal performance cost, so it's usually the better choice for binaries.

#### Generic Traits

Rust traits can be generic in one of two ways: with generic type parameters like trait `Foo<T>` or with associated types like `trait Foo { type Bar; }`.
The difference between these is not immediately apparent, but luckily the rule of thumb is quite simple: use an associated type if you expect only one implementation of the trait for a given type, and use a generic type parameter otherwise.

The rationale for this is that associated types are often significantly easier to work with, but will not allow multiple implementations.
So, more simply put, the advice is really just to use associated types whenever you can.

#### Coherence and the Orphan Rule

Rust has some fairly strict rules about where you can implement traits and what types you can implement them on.
These rules exist to preserve the coherence property: for any given type and method, there is only ever one correct choice for which implementation of the method to use for that type.

#### Trait Bounds

The standard library is flush with trait bounds, whether it's that the keys in a `HashMap` must implement `Hash + Eq` or that the function given to `thread::spawn` must be `FnOnce + Send + 'static`.
When you write generic code yourself, it will almost certainly include trait bounds, as otherwise your code cannot do much with the type it is generic over.
As you write more elaborate generic implementations, you'll find that you also need more fidelity from your trait bounds, so let's look at some of the ways to achieve that.

#### Marker Traits

Some traits, called *marker traits*, instead indicate a property of the implementing type.
Marker traits have no methods or associated types and serve just to tell you that a particular type can or cannot be used in a certain way.
