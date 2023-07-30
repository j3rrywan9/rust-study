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

Rust allows the owner of a value to lend out that value to others, without giving up ownership, through references.
*References* are pointers that come with an additional contract for how they can be used, such as whether the reference provides exclusive access to the referenced value, or whether the referenced value may also have other references point to it.

#### Shared References

A shared reference, `&T`, is, as the name implies, a pointer that may be shared.
Any number of other references may exist to the same value, and each shared reference is `Copy`, so you can trivially make more of them.
Values behind shared references are not mutable; you cannot modify or reassign the value a shared reference points to, nor can you cast a shared reference to a mutable one.

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

Before we talk about how a type's in-memory representation is determined, we first need to discuss the notion of *alignment*, which dictates where the bytes for a type can be stored.

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
Specifically, it is twice the size of a `usize` (the size of a word on the target platform): one `usize` for holding the pointer, and one `usize` for holding the extra information needed to "complete" the type.

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
Well, with dynamic dispatch, the caller simply tells you.
If you replace `impl Pattern` with `&dyn Pattern`, you tell the caller that they must give *two* pieces of information for this argument:
the address of the pattern *and* the address of the `is_contained_in` method.
In practice, the caller gives us a pointer to a chunk of memory called a virtual method table, or *vtable*, that holds the address of the implementation of *all* the trait's methods for the type in question, one of which is `is_contained_in`.
When the code inside the method wants to call a trait method on the provided pattern, it looks up the address of that pattern's implementation of `is_contained_in` in the vtable and then calls the function at that address.
This allows us to use the same function body regardless of what type the caller wants to use.

You'll notice that when we opted in to dynamic dispatch using the `dyn` keyword, we had to place an `&` in front of it.
The reason is that we no longer know at compile time the size of the pattern type that the caller passes in, so we don't know how much space to set aside for it.
In other words, `dyn Trait` is `!Sized`, where the `!` means not.
To make it `Sized` so we can take it as an argument, we place it behind a pointer (which we know the size of).
Since we also need to pass along the table of method addresses, this pointer becomes a wide pointer, where the extra word holds the pointer to the vtable.
You can use any type that is able to hold a wide pointer for dynamic dispatch, such as `&mut`, `Box`, and `Arc`.
Listing 2-3 shows the dynamic dispatch equivalent of Listing 2-2.
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
With dynamic dispatch, all the compiler can do for `contains` in Listing 2-3 is insert a call to the function through the vtable - it can no longer perform any additional optimizations as it does not know what code will sit on the other side of that function call.
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

With a generic trait, users must always specify all the generic parameters and repeat any bounds on those parameters.
This can quickly get messy and hard to maintain.
If you add a generic parameter to a trait, all users of that trait must also be updated to reflect the change.
And since multiple implementations of a trait may exist for a given type, the compiler may have a hard time deciding which instance of the trait you meant to use, leading to awful disambiguating function calls like `FromIterator::<u32>::from_iter`.

#### Coherence and the Orphan Rule

Rust has some fairly strict rules about where you can implement traits and what types you can implement them on.
These rules exist to preserve the coherence property: for any given type and method, there is only ever one correct choice for which implementation of the method to use for that type.

#### Trait Bounds

The standard library is flush with trait bounds, whether it's that the keys in a `HashMap` must implement `Hash + Eq` or that the function given to `thread::spawn` must be `FnOnce + Send + 'static`.
When you write generic code yourself, it will almost certainly include trait bounds, as otherwise your code cannot do much with the type it is generic over.
As you write more elaborate generic implementations, you'll find that you also need more fidelity from your trait bounds, so let's look at some of the ways to achieve that.

First and foremost, trait bounds do not have to be of the form `T: Trait` where `T` is some type your implementation or type is generic over.
The bounds can be arbitrary type restrictions and do not even need to include generic parameters, types of arguments, or local types.
You can write a trait bound like `where String: Clone`, even though `String: Clone` is always true and contains no local types.
You can also write `where io::Error: From<MyError<T>>`; your generic type parameters do not need to appear only on the left-hand side.
This not only allows you to express more intricate bounds but also can save you from needlessly repeating bounds.

Sometimes, you want bounds on associated types of types you're generic over.
As an example, consider the iterator method `flatten`, which takes an iterator that produces items that in turn implement `Iterator` and produces an iterator of the items of those inner iterators.
The type it produces, `Flatten`, is generic over `I`, which is the type of the outer iterator.
`Flatten` implements `Iterator` if `I` implements `Iterator` *and* the items yielded by `I` themselves implement `IntoIterator`.
To enable you to write bounds like this, Rust lets you refer to associated types of a type using the syntax `Type::AssocType`.
For example, we can refer to `I`'s Item type using `I::Item`.
If a type has multiple associated types by the same name, such as if the trait that provides the associated type is itself generic (and therefore there are many implementations), you can disambiguate with the syntax `<Type as Trait>::AssocType`.
Using this, you can write bounds not only for the outer iterator type but also for the item type of that outer iterator.

In code that uses generics extensively, you may find that you need to write a bound that talks about references to a type.
This is normally fine, as you'll tend to also have a generic lifetime parameter that you can use as the lifetime for these references.
In some cases, however, you want the bound to say "this reference implements this trait for any lifetime."
This type of bound is known as a *higher-ranked trait bound*, and it's particularly useful in association with the `Fn` traits.

#### Marker Traits

Some traits, called *marker traits*, instead indicate a property of the implementing type.
Marker traits have no methods or associated types and serve just to tell you that a particular type can or cannot be used in a certain way.

## Chapter 3: Designing Interfaces

Every project, no matter how large or small, has an API.

In this chapter we'll look at some of the most important considerations for writing idiomatic interfaces in Rust, whether the users of those interfaces are your own code or other developers using your library.
These essentially boil down to four principles: your interfaces should be *unsurprising*, *flexible*, *obvious*, and *constrained*.
I'll discuss each of these principles in turn, to provide some guidance for writing reliable and usable interfaces.

I highly recommend taking a look at the Rust API Guidelines (https://rust-lang.github.io/api-guidelines/) after you've read this chapter.
There's an excellent checklist you can follow, with a detailed run-through of each recommendation.

### Unsurprising

The Principle of Least Surprise, otherwise known as the Law of Least Astonishment, comes up a lot in software engineering, and it holds true for Rust interfaces as well.
Where possible, your interfaces should be intuitive enough that if the user has to guess, they usually guess correctly.
Of course, not everything about your application is going to be immediately intuitive in this way, but anything that *can* be unsurprising should be.
The core idea here is to stick close to things the user is likely to already know so that they don't have to relearn concepts in a different way than they're used to.
That way you can save their brain power for figuring out the things that are actually specific to your interface.

There are a variety of ways you can make your interfaces predictable.
Here, we'll look at how you can use naming, common traits, and ergonomic trait tricks to help the user out.

#### Naming Practices

A user of your interface will encounter it first through its names; they will immediately start to infer things from the names of types, methods, variables, fields, and libraries they come across.
If your interface reuses names for things - say, methods and types - from other (perhaps common) interfaces, the user will know they can make certain assumptions about your methods and types.
A method called `into_inner` probably takes `self` and likely returns some kind of wrapped type.
A type called `SomethingError` probably implements `std::error::Error` and appears in various `Result`s.
By reusing common names for the same purpose, you make it easier for the user to guess what things do and allow them to more easily understand the things that are different about your interface.

A corollary to this is that things that share a name *should* in fact work the same way.

#### Common Traits for Types

Users in Rust will also make the major assumption that everything in the interface "just works."
They expect to be able to print any type with `{:?}` and send anything and everything to another thread, and they expect that every type is `Clone`.
Where possible, we should again avoid surprising the user and eagerly implement most of the standard traits even if we do not need them immediately.

First among these standard traits is the `Debug` trait.
Nearly every type can, and should, implement `Debug`, even if it only prints the type's name.
Using `#[derive(Debug)]` is often the best way to implement the `Debug` trait in your interface, but keep in mind that all derived traits automatically add the same bound for any generic parameters.
You could also simply write your own implementation by leveraging the various `debug_` helpers on `fmt::Formatter`.

Tied in close second are the Rust auto-traits `Send` and `Sync` (and, to a lesser extent, `Unpin`).

The next set of nearly universal traits you should implement is `Clone` and `Default`.
These traits can be derived or implemented easily and make sense to implement for most types.
If your type cannot implement these traits, make sure to call it out in your documentation, as users will usually expect to be able to easily create more (and new) instances of types as they see fit.
If they cannot, they will be surprised.

#### Ergonomic Trait Implementations

Iterators are another case where you'll often want to specifically add trait implementations on references to a type.
For any type that can be iterated over, consider implementing `IntoIterator` for both `&MyType` and `&mut MyType` where applicable.
This makes for loops work with borrowed instances of your type as well out of the box, just like users would expect.

#### Wrapper Types

Rust does not have object inheritance in the classical sense.
However, the `Deref` trait and its cousin `AsRef` both provide something a little like inheritance.
These traits allow you to have a value of type `T` and call methods on some type `U` by calling them directly on the `T`-typed value if `T: Deref<Target = U>`.
This feels like magic to the user, and is generally great.

### Flexible

Every piece of code you write includes, implicitly or explicitly, a contract.
The contract consists of a set of requirements and a set of promises.
The requirements are restrictions on how the code can be used, while the promises are guarantees about how the code can be used.
When designing a new interface, you want to think carefully about this contract.
A good rule of thumb is to avoid imposing unnecessary restrictions and to only make promises you can keep.
Adding restrictions or removing promises usually requires a major semantic version change and is likely to break code elsewhere.
Relaxing restrictions or giving additional promises, on the other hand, is usually backward compatible.

In Rust, restrictions usually come in the form of trait bounds and argument types, and promises come in the form of trait implementations and return types.

#### Generic Arguments

One obvious requirement your interface must place on users is what types they must provide to your code.
If your function explicitly takes a `Foo`, the user must own and give you a `Foo`.
There is no way around it.
In most cases it pays off to use generics rather than concrete types, to allow the caller to pass any type that conforms to what your function actually needs, rather than only a particular type.
One way to go about relaxing requirements this way is to start with the argument fully generic with no bounds, and then just follow the compiler errors to discover what bounds you need to add.

#### Object Safety

#### Borrowed vs. Owned

For nearly every function, trait, and type you define in Rust, you must decide whether it should own, or just hold a reference to, its data.
Whatever decision you make will have far-reaching implications for the ergonomics and performance of your interface.
Luckily, these decisions very often make themselves.

If the code you write needs ownership of the data, such as to call methods that take `self` or to move the data to another thread, it must store the owned data.
When your code must own data, it should generally also make the caller provide owned data, rather than taking values by reference and cloning them.
This leaves the caller in control of allocation, and it is upfront about the cost of using the interface in question.

Sometimes, you don't know if your code must own data or not, as it is runtime dependent.
For this, the Cow type is your friend.
It lets you represent data that *may* be owned by holding either a reference or an owned value.
If asked to produce an owned value when it only has a reference, a `Cow` uses the `ToOwned` trait to make one behind the scenes, usually by cloning.
`Cow` is typically used in return types to represent functions that sometimes allocate.

Other times, reference lifetimes complicate the interface so much that it becomes a pain to use.
If your users are struggling to get code to compile on top of your interface, that's a sign that you may want to (even unnecessarily) take ownership of certain pieces of data.
If you do this, start with data that is cheap to clone or is not part of anything performance-sensitive before you decide to heap-allocate what might be a huge chunk of bytes.

## Chapter 8: Asynchronous Programming

### Ergonomic Futures

Writing a type that implements `Future` in the way I've described so far is quite a pain.

#### `async`/`await`

To do that, we first need to talk about *generators* - the mechanism by which `async` and `await` are implemented.

##### Generators

Briefly described, a generator is a chunk of code with some extra compiler-generated bits that enables it to stop, or *yield*, its execution midway through and then resume from where it last yielded later on.

#### `Pin` and `Unpin`

We're not quite done.
While generators are neat, a challenge arises from the technique as I've described it so far.
In particular, it's not clear what happens if the code in the generator (or, equivalently, the `async` block) takes a reference to a local variable.

## Chapter 9: Unsafe Code

Your main takeaway from this chapter should be this: unsafe code is the mechanism Rust gives developers for taking advantage of invariants that, for whatever reason, the compiler cannot check.

Crucially, unsafe code is not a way to skirt the various rules of Rust, like borrow checking, but rather a way to enforce those rules using reasoning that is beyond the compiler.
When you write unsafe code, the onus is on you to ensure that the resulting code is safe.
In a way, `unsafe` is misleading as a keyword when it is used to allow unsafe operations through `unsafe {}`; it's not that the contained code is unsafe, it's that the code is allowed to perform otherwise unsafe operations because in this particular context, those operations *are* safe.

### The `unsafe` Keyword

Before we discuss the powers that `unsafe` grants you, we need to talk about its two different meanings.
The `unsafe` keyword serves a dual purpose in Rust: it marks a particular function as unsafe to call *and* it enables you to invoke unsafe functionality in a particular code block.

## Chapter 11: Foreign Function Interfaces

Not all code is written in Rust.
It's shocking, I know.
Every so often, you'll need to interact with code written in other languages, either by calling into such code from Rust or by allowing that code to call your Rust code.
You can achieve this through *foreign function interfaces (FFI)*.
