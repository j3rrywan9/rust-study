# The Rust Programming Language

## 7. Managing Growing Projects With Packages, Crates, And Modules

### Packages and Crates

The first parts of the module system we'll cover are packages and crates.

A crate can come in one of two forms: a binary crate or a library crate.
A package can contain as many binary crates as you like, but at most only one library crate.
A package must contain at least one crate, whether that's a library or binary crate.

Cargo follows a convention that *src/main.rs* is the crate root of a binary crate with the same name as the package.
Likewise, Cargo knows that if the package directory contains *src/lib.rs*, the package contains a library crate with the same name as the package, and *src/lib.rs* is its crate root.
Cargo passes the crate root files to rustc to build the library or binary.

### Defining Modules to Control Scope and Privacy

In this section, we'll talk about modules and other parts of the module system, namely *paths*, which allow you to name items; the `use` keyword that brings a path into scope; and the `pub` keyword to make items public.
We'll also discuss the `as` keyword, external packages, and the glob operator.

*Modules* let us organize code within a crate for readability and easy reuse.
Modules also allow us to control the *privacy* of items because code within a module is private by default.
Private items are internal implementation details not available for outside use.
We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.

Earlier, we mentioned that *src/main.rs* and *src/lib.rs* are called crate roots.
The reason for their name is that the contents of either of these two files form a module named `crate` at the root of the crate's module structure, known as the *module tree*.

The module tree might remind you of the filesystem's directory tree on your computer; this is a very apt comparison!
Just like directories in a filesystem, you use modules to organize your code.
And just like files in a directory, we need a way to find our modules.

### Paths for Referring to an Item in the Module Tree

To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem.
To call a function, we need to know its path.

#### Exposing Paths with the `pub` Keyword

#### Starting Relative Paths with `super`

#### Making Structs and Enums Public

We can also use `pub` to designate structs and enums as public, but there are a few extra details to the usage of pub with structs and enums.
If we use `pub` before a struct definition, we make the struct public, but the struct's fields will still be private.
We can make each field public or not on a case-by-case basis.

In contrast, if we make an enum public, all of its variants are then public.
We only need the `pub` before the `enum` keyword

### Bring Paths into Scope with the `use` Keyword

#### Creating Idiomatic `use` Paths

#### Providing New Names with the `as` Keyword

#### Re-exporting Names with `pub use`

When we bring a name into scope with the `use` keyword, the name available in the new scope is private.
To enable the code that calls our code to refer to that name as if it had been defined in that code's scope, we can combine `pub` and `use`.
This technique is called *re-exporting* because we're bringing an item into scope but also making that item available for others to bring into their scope.

Listing 7-17 shows the code in Listing 7-11 with use in the root module changed to `pub use`.

**src/lib.rs**
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
Listing 7-17: Making a name available for any code to use from a new scope with `pub use`

Before this change, external code would have to call the `add_to_waitlist` function by using the path `restaurant::front_of_house::hosting::add_to_waitlist()`.
Now that this `pub use` has re-exported the `hosting` module from the root module, external code can use the path `restaurant::hosting::add_to_waitlist()` instead.

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain.
For example, in this restaurant metaphor, the people running the restaurant think about "front of house" and "back of house."
But customers visiting a restaurant probably won't think about the parts of the restaurant in those terms.
With `pub use`, we can write our code with one structure but expose a different structure.
Doing so makes our library well organized for programmers working on the library and programmers calling the library.

## 10. Generic Types, Traits, and Lifetimes

Every programming language has tools for effectively handling the duplication of concepts.
In Rust, one such tool is *generics*: abstract stand-ins for concrete types or other properties.
We can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.

Functions can take parameters of some generic type, instead of a concrete type, like `i32` or `String`, in the same way a function takes parameters with unknown values to run the same code on multiple concrete values.

### Removing Duplication by Extracting a Function

Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication.

### 10.1 Generic Data Types

We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.
Let's first look at how to define functions, structs, enums, and methods using generics.
Then we'll discuss how generics affect code performance.

#### In Function Definitions

To parameterize the types in a new single function, we need to name the type parameter, just as we do for the value parameters to a function.
You can use any identifier as a type parameter name.
But we'll use `T` because, by convention, type parameter names in Rust are short, often just a letter, and Rust's type-naming convention is CamelCase.
Short for "type," `T` is the default choice of most Rust programmers.

When we use a parameter in the body of the function, we have to declare the parameter name in the signature so the compiler knows what that name means.
Similarly, when we use a type parameter name in a function signature, we have to declare the type parameter name before we use it.
To define the generic largest function, place type name declarations inside angle brackets, `<>`, between the name of the function and the parameter list, like this:
```rust
fn largest<T>(list: &[T]) -> &T {
```

#### In Struct Definitions

#### In Enum Definitions

As we did with structs, we can define enums to hold generic data types in their variants.
Let's take another look at the `Option<T>` enum that the standard library provides, which we used in Chapter 6:
```rust
enum Option<T> {
    Some(T),
    None,
}
```
This definition should now make more sense to you.
As you can see, the `Option<T>` enum is generic over type `T` and has two variants: `Some`, which holds one value of type `T`, and a `None` variant that doesn't hold any value.
By using the `Option<T>` enum, we can express the abstract concept of an optional value, and because `Option<T>` is generic, we can use this abstraction no matter what the type of the optional value is.

#### In Method Definitions

We can implement methods on structs and enums (as we did in Chapter 5) and use generic types in their definitions, too.
Listing 10-9 shows the `Point<T>` struct we defined in Listing 10-6 with a method named `x` implemented on it.
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```
Here, we've defined a method named `x` on `Point<T>` that returns a reference to the data in the field `x`.

Note that we have to declare `T` just after `impl` so we can use `T` to specify that we're implementing methods on the type `Point<T>`.
By declaring `T` as a generic type after `impl`, Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete type.
We could have chosen a different name for this generic parameter than the generic parameter declared in the struct definition, but using the same name is conventional.
Methods written within an `impl` that declares the generic type will be defined on any instance of the type, no matter what concrete type ends up substituting for the generic type.

We can also specify constraints on generic types when defining methods on the type.
We could, for example, implement methods only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type.
In Listing 10-10 we use the concrete type `f32`, meaning we don't declare any types after `impl`.
```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

#### Performance of Code Using Generics

### 10.2 Traits: Defining Shared Behavior

A *trait* defines functionality a particular type has and can share with other types.
We can use traits to define shared behavior in an abstract way.
We can use *trait bounds* to specify that a generic type can be any type that has certain behavior.

#### Defining a Trait

A type's behavior consists of the methods we can call on that type.
Different types share the same behavior if we can call the same methods on all of those types.
Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.

#### Implementing a Trait on a Type

## 15. Smart Pointers

A *pointer* is a general concept for a variable that contains an address in memory.
This address refers to, or "points at," some other data.
The most common kind of pointer in Rust is a reference, which you learned about in Chapter 4.
References are indicated by the `&` symbol and borrow the value they point to.
They don't have any special capabilities other than referring to data, and have no overhead.

*Smart pointers*, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.
The concept of smart pointers isn't unique to Rust: smart pointers originated in C++ and exist in other languages as well.
Rust has a variety of smart pointers defined in the standard library that provide functionality beyond that provided by references.
To explore the general concept, we'll look at a couple of different examples of smart pointers, including a *reference counting* smart pointer type.
This pointer enables you to allow data to have multiple owners by keeping track of the number of owners and, when no owners remain, cleaning up the data.

Rust, with its concept of ownership and borrowing, has an additional difference between references and smart pointers: while references only borrow data, in many cases, smart pointers *own* the data they point to.

Though we didn't call them as such at the time, we've already encountered a few smart pointers in this book, including `String` and `Vec<T>` in Chapter 8.
Both these types count as smart pointers because they own some memory and allow you to manipulate it.
They also have metadata and extra capabilities or guarantees.
`String`, for example, stores its capacity as metadata and has the extra ability to ensure its data will always be valid UTF-8.

Smart pointers are usually implemented using structs.
Unlike an ordinary struct, smart pointers implement the `Deref` and `Drop` traits.
The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers.
The `Drop` trait allows you to customize the code that's run when an instance of the smart pointer goes out of scope.
In this chapter, we'll discuss both traits and demonstrate why they're important to smart pointers.

Given that the smart pointer pattern is a general design pattern used frequently in Rust, this chapter won't cover every existing smart pointer.
Many libraries have their own smart pointers, and you can even write your own.
We'll cover the most common smart pointers in the standard library:
* `Box<T>` for allocating values on the heap
* `Rc<T>`, a reference counting type that enables multiple ownership
* `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time

In addition, we'll cover the *interior mutability* pattern where an immutable type exposes an API for mutating an interior value.
We'll also discuss *reference cycles*: how they can leak memory and how to prevent them.

Let's dive in!

### Using `Box<T>` to Point to Data on the Heap

The most straightforward smart pointer is a *box*, whose type is written `Box<T>`.
Boxes allow you to store data on the heap rather than the stack.
What remains on the stack is the pointer to the heap data.
Refer to Chapter 4 to review the difference between the stack and the heap.

Boxes don't have performance overhead, other than storing their data on the heap instead of on the stack.
But they don't have many extra capabilities either.
You'll use them most often in these situations:
* When you have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size
* When you have a large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so
* When you want to own a value and you care only that it's a type that implements a particular trait rather than being of a specific type
