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
If the result is an `Err(e)`, `expect` prints a message that includes a description of `e` and exits the program immediately.
However, if the result is `Ok(v)`, `expect` simply returns `v` itself, which we are finally able to push onto the end of our vector of numbers.

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

A *tuple* is a pair, or triple, quadruple, quintuple, etc. (hence, *n-tuple*, or *tuple*), of values of assorted types.

Rust code often uses tuple type to return multiple values from a function.

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

String literals are enclosed in double quotes.

For these cases, Rust offers *raw strings*.
A raw string is tagged with the lowercase letter `r`.

#### Byte Strings

#### Strings in Memory

Rust strings are sequences of Unicode characters, but they are not stored in memory as arrays of `char`s.
Instead, they are stored using UTF-8, a variable-width encoding.
Each ASCII character in a string is stored in one byte.
Other characters take up multiple bytes.

A `String` has a resizable buffer holding UTF-8 text.
The buffer is allocated on the heap, so it can resize its buffer as needed or requested.
You can think of a `String` as a `Vec<u8>` that is guaranteed to hold well-formed UTF-8; in fact, this is how `String` is implemented.

A `&str` (pronounced "stir" or "string slice") is a reference to a run of UTF-8 text owned by someone else: it "borrows" the text.

A string literal is a `&str` that refers to preallocated text, typically stored in read-only memory along with the program's machine code.

## Chapter 4. Ownership and Moves

### Ownership

In Rust, however, the concept of ownership is built into the language itself and enforced by compile-time checks.
Every value has a single owner that determines its lifetime.
When the owner is freed - *dropped*, in Rust terminology - the owned value is dropped too.
These rules are meant to make it easy for you to find any given value's lifetime simply by inspecting the code, giving you the control over its lifetime that a systems language should provide.

A variable owns its value.
When control leaves the block in which the variable is declared, the variable is dropped, so its value is dropped along with it.

It follows that the owners and their owned values form *trees*: your owner is your parent, and the values you own are your children.
And at the ultimate root of each tree is a variable; when that variable goes out of scope, the entire tree goes with it.

Rust programs don't usually explicitly drop values at all, in the way C and C++ programs would use `free` and `delete`.
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

#### More Operations That Move

In the examples thus far, we've shown initializations, providing values for variables as they come into scope in a `let` statement.
Assigning to a variable is slightly different, in that if you move a value into a variable that was already initialized, Rust drops the variable's prior value.
For example:
```rust
let mut s = "Govinda".to_string();
s = "Siddhartha".to_string(); // value "Govinda" dropped here
```
Moving values around like this may sound inefficient, but there are two things to keep in mind.
First, the moves always apply to the value proper, not the heap storage they own.
For vectors and strings, the value proper is the three-word header alone; the potentially large element arrays and text buffers sit where they are in the heap.
Second, the Rust compiler's code generation is good at "seeing through" all these moves; in practice, the machine code often stores the value directly where it belongs.

#### Moves and Control Flow

The previous examples all have very simple control flow; how do moves interact with more complicated code?
The general principle is that, if it's possible for a variable to have had its value moved away and it hasn't definitely been given a new value since, it's considered uninitialized.

#### Moves and Indexed Content

We've mentioned that a move leaves its source uninitialized, as the destination takes ownership of the value.
But not every kind of value owner is prepared to become uninitialized.

### Copy Types: The Exception to Moves

Earlier we were careful to say that *most* types are moved; now we've come to the exceptions, the types Rust designates as `Copy` types.
Assigning a value of a `Copy` type copies the value, rather than moving it.
The source of the assignment remains initialized and usable, with the same value it had before.
Passing `Copy` types to functions and constructors behaves similarly.

The standard `Copy` types include all the machine integer and floating-point numeric types, the `char` and `bool` types, and a few others.
A tuple or fixed-size array of `Copy` types is itself a `Copy` type.

Only types for which a simple bit-for-bit copy suffices can be `Copy`.
As we've already explained, `String` is not a `Copy` type, because it owns a heap-allocated buffer.
For similar reasons, `Box<T>` is not `Copy`; it owns its heap-allocated referent.
The `File` type, representing an operating system file handle, is not `Copy`; duplicating such a value would entail asking the operating system for another file handle.
Similarly, the `MutexGuard` type, representing a locked mutex, isn't `Copy`: this type isn't meaningful to copy at all, as only one thread may hold a mutex at a time.

As a rule of thumb, any type that needs to do something special when a value is dropped cannot be `Copy`: a `Vec` needs to free its elements, a `File` needs to close its file handle, a `MutexGuard` needs to unlock its mutex, and so on.
Bit-for-bit duplication of such types would leave it unclear which value was now responsible for the original's resources.

What about types you define yourself?
By default, `struct` and `enum` types are not `Copy`:
```rust
struct Label { number: u32 }

fn print(l: Label) { println!("STAMP: {}", l.number); }

let l = Label { number: 3 };
print(l);
println!("My label number is: {}", l.number);
```
This won't compile;

But user-defined types being non-`Copy` is only the default.
If all the fields of your struct are themselves `Copy`, then you can make the type `Copy` as well by placing the attribute `#[derive(Copy, Clone)]` above the definition, like so:
```rust
#[derive(Copy, Clone)]
struct Label { number: u32 }
```
With this change, the preceding code compiles without complaint.

One of Rust's principles is that costs should be apparent to the programmer.
Basic operations must remain simple.
Potentially expensive operations should be explicit, like the calls to `clone` in the earlier example that make deep copies of vectors and the strings they contain.

### `Rc` and `Arc`: Shared Ownership

Although most values have unique owners in typical Rust code, in some cases it's difficult to find every value a single owner that has the lifetime you need; you'd like the value to simply live until everyone's done using it.
For these cases, Rust provides the reference-counted pointer types `Rc` and `Arc`.
As you would expect from Rust, these are entirely safe to use: you cannot forget to adjust the reference count, create other pointers to the referent that Rust doesn't notice, or stumble over any of the other sorts of problems that accompany reference-counted pointer types in C++.

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

```rust
use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}
```
In particular, `HashMap` is not `Copy` - it can't be, since it owns a dynamically allocated table.
So when the program calls `show(table)`, the whole structure gets moved to the function, leaving the variable table uninitialized.
(It also iterates over its contents in no specific order, so if you've gotten a different order, don't worry.)
If the calling code tries to use `table` now, it'll run into trouble:
```rust
...
show(table);
assert_eq!(table["Gesualdo"][0], "many madrigals");
```
The right way to handle this is to use references.
A reference lets you access a value without affecting its ownership.
References come in two kinds:
* A *shared reference* lets you read but not modify its referent.
However, you can have as many shared references to a particular value at a time as you like.
The expression `&e` yields a shared reference to `e`'s value; if `e` has the type `T`, then `&e` has the type `&T`, pronounced "ref `T`."
Shared references are `Copy`.
* If you have *mutable reference* to a value, you may both read and modify the value.
However, you may not have any other references of any sort to that value active at the same time.
The expression `&mut e` yields a mutable reference to `e`'s value; you write its type as `&mut T`, which is pronounced "ref mute `T`."
Mutable references are not `Copy`.

You can think of the distinction between shared and mutable references as a way to enforce a *multiple readers or single writer* rule at compile time.
In fact, this rule doesn't apply only to references; it covers the borrowed value's owner as well.
As long as there are shared references to a value, not even its owner can modify it; the value is locked down.
Nobody can modify `table` while `show` is working with it.
Similarly, if there is a mutable reference to a value, it has exclusive access to the value; you can't use the owner at all, until the mutable reference goes away.
Keeping sharing and mutation fully separate turns out to be essential to memory safety, for reasons we'll go into later in the chapter.

The printing function in our example doesn't need to modify the table, just read its contents.
So the caller should be able to pass it a shared reference to the table, as follows:
```rust
show(&table);
```
References are non-owning pointers, so the table variable remains the owner of the entire structure; `show` has just borrowed it for a bit.

When we pass a value to a function in a way that moves ownership of the value to the function, we say that we have passed it *by value*.
If we instead pass the function a reference to the value, we say that we have passed the value *by reference*.

### Working with References

The preceding example shows a pretty typical use for references: allowing functions to access or manipulate a structure without taking ownership.
But references are more flexible than that, so let's look at some examples to get a more detailed view of what's going on.

#### Rust References Versus C++ References

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

#### Assigning References

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

#### References to References

Rust permits references to references:

#### Comparing References

Like the `.` operator, Rust's comparison operators "see through" any number of references:
```rust
let x = 10;
let y = 10;

let rx = &x;
let ry = &y;

let rrx = &rx;
let rry = &ry;

assert!(rrx <= rry);
assert!(rrx == rry);
```
The final assertion here succeeds, even though `rrx` and `rry` point at different values (namely, `rx` and `ry`), because the `==` operator follows all the references and performs the comparison on their final targets, `x` and `y`.
This is almost always the behavior you want, especially when writing generic functions.
If you actually want to know whether two references point to the same memory, you can use `std::ptr::eq`, which compares them as addresses:
```rust
assert!(rx == ry);              // their referents are equal
assert!(!std::ptr::eq(rx, ry)); // but occupy different addresses
```
Note that the operands of a comparison must have exactly the same type, including the references:
```rust
assert!(rx == rrx);    // error: type mismatch: `&i32` vs `&&i32`
assert!(rx == *rrx);   // this is okay
```

### References Are Never Null

Rust references are never null.
There's no analogue to C's `NULL` or C++'s `nullptr`.
There is no default initial value for a reference (you can't use any variable until it's been initialized, regardless of its type) and Rust won't convert integers to references (outside of `unsafe` code), so you can't convert zero into a reference.

In Rust, if you need a value that is either a reference to something or not, use the type `Option<&T>`.
At the machine level, Rust represents `None` as a null pointer and `Some(r)`, where `r` is a `&T` value, as the nonzero address, so `Option<&T>` is just as efficient as a nullable pointer in C or C++, even though it's safer: its type requires you to check whether it's `None` before you can use it.

### Borrowing References to Arbitrary Expressions

### References to Slices and Trait Objects

The references we've shown so far are all simple addresses.
However, Rust also includes two kinds of *fat pointers*, two-word values carrying the address of some value, along with some further information necessary to put the value to use.

A reference to a slice is a fat pointer, carrying the starting address of the slice and its length.

Rust's other kind of fat pointer is a *trait object*, a reference to a value that implements a certain trait.
A trait object carries a value's address and a pointer to the trait's implementation appropriate to that value, for invoking the trait's methods.

Aside from carrying this extra data, slice and trait object references behave just like the other sorts of references we've shown so far in this chapter: they don't own their referents, they are not allowed to outlive their referents, they may be mutable or shared, and so on.

### Reference Safety

To convey the fundamental ideas, we'll start with the simplest cases, showing how Rust ensures references are used properly within a single function body.
Then we'll look at passing references between functions and storing them in data structures.
This entails giving said functions and data types *lifetime parameters*, which we'll explain.
Finally, we'll present some shortcuts that Rust provides to simplify common usage patterns.
Throughout, we'll be showing how Rust points out broken code and often suggests solutions.

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

Beyond the point where `x` goes out of scope, the reference would be a dangling pointer.
We say that the variable's lifetime must *contain* or *enclose* that of the reference borrowed from it.
```rust
{
    let r;
    {
        let x = 1;
        ...
        r = &x;
        ...
    }
    assert_eq!(*r, 1);
}
```
Here's another kind of constraint: if you store a reference in a variable `r`, the reference's type must be good for the entire lifetime of the variable, from its initialization until its last use, as shown in Figure 5-4.

If the reference can't live at least as long as the variable does, then at some point `r` will be a dangling pointer.
We say that the reference's lifetime must *contain* or *enclose* the variable's.

The first kind of constraint limits how large a reference's lifetime can be, while the second kind limits how small it can be.
Rust simply tries to find a lifetime for each reference that satisfies all these constraints.

These rules apply in a natural way when you borrow a reference to some part of some larger data structure, like an element of a vector:
```rust
let v = vec![1, 2, 3];
let r = &v[1];
```

#### Receiving References as Function Arguments

When we pass a reference to a function, how does Rust make sure the function uses it safely?

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
Since `STASH` lives for the program's entire execution, the reference type it holds must have a lifetime of the same length; Rust calls this the `'static` lifetime.
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

You only need to worry about lifetime parameters when defining functions and types; when using them, Rust infers the lifetimes for you.

#### Returning References

It's common for a function to take a reference to some data structure and then return a reference into some part of that structure.

Lifetimes in function signatures let Rust assess the relationships between the references you pass to the function and those the function returns, and they ensure they're being used safely.

#### Structs Containing References

Whenever a reference type appears inside another type's definition, you must write out its lifetime.

The alternative is to give the type a lifetime parameter `'a` and use that for `r`:
```rust
struct S<'a> {
    r: &'a i32
}
```

#### Distinct Lifetime Parameters

The problem arises because both references in `S` have the same lifetime `'a`.
Changing the definition of `S` to let each reference have a distinct lifetime fixes everything:
```rust
struct S<'a, 'b> {
    x: &'a i32,
    y: &'b i32
}
```

#### Omitting Lifetime Parameters

We've shown plenty of functions so far in this book that return references or take them as parameters, but we've usually not needed to spell out which lifetime is which.
The lifetimes are there; Rust is just letting us omit them when it's reasonably obvious what they should be.

### Sharing Versus Mutation

## Chapter 6. Expressions

### An Expression Language

Expressions have values.
Statements don't.

Rust is what is called an *expression language*.
This means it follows an older tradition, dating back to Lisp, where expressions do all the work.

### Precedence and Associativity

### Blocks and Semicolons

Blocks are the most general kind of expression.
A block produces a value and can be used anywhere a value is needed:
```rust
let display_name = match post.author() {
    Some(author) => author.name(),
    None => {
        let network_info = post.get_network_metadata()?;
        let ip = network_info.client_address();
        ip.to_string()
    }
};
```
Note that there is no semicolon after the `ip.to_string()` method call.
Most lines of Rust code do end with either a semicolon or curly braces, just like C or Java.
And if a block looks like C code, with semicolons in all the familiar places, then it will run just like a C block, and its value will be `()`.
As we mentioned in Chapter 2, when you leave the semicolon off the last line of a block, that makes the value of the block the value of its final expression, rather than the usual `()`.

### Declarations

You may occasionally see code that seems to redeclare an existing variable, like this:
```rust

```

### `if` and `match`

The form of an `if` expression is familiar:
```rust
if condition1 {
    block1
} else if condition2 {
    block2
} else {
    block_n
}
```
Each `condition` must be an expression of type `bool`; true to form, Rust does not implicitly convert numbers or pointers to Boolean values.

Rust prohibits `match` expressions that do not cover all possible values:
```rust
let score = match card.rank {
    Jack => 10,
    Queen => 10,
    Ace => 11
};  // error: nonexhaustive patterns
```

### `if let`

There is one more `if` form, the `if let` expression:
```rust
if let pattern = expr {
    block1
} else {
    block2
}
```
The given `expr` either matches the `pattern`, in which case `block1` runs, or doesn't match, and `block2` runs.
Sometimes this is a nice way to get data out of an `Option` or `Result`:
```rust
if let Some(cookie) = request.session_cookie {
    return restore_session(cookie);
}

if let Err(err) = show_cheesy_anti_robot_task() {
    log_robot_attempt(err);
    politely_accuse_user_of_being_a_robot();
} else {
    session.mark_as_human();
}
```
It's never strictly necessary to use `if let`, because `match` can do everything `if let` can do.
An `if let` expression is shorthand for a `match` with just one pattern:
```rust
match expr {
    pattern => { block1 }
    _ => { block2 }
}
```

### Loops

There are four looping expressions:
```rust
while condition {
    block
}

while let pattern = expr {
    block
}

loop {
    block
}

for pattern in iterable {
    block
}
```
Loops are expressions in Rust, but the value of a `while` or `for` loop is always `()`, so their value isn't very useful.
A `loop` expression can produce a value if you specify one.

### Control Flow in Loops

A `break` expression exits an enclosing loop.
(In Rust, `break` works only in loops. It is not necessary in `match` expressions, which are unlike `switch` statements in this regard.)

### `return` Expressions

A `return` expression exits the current function, returning a value to the caller.

### Why Rust Has `loop`

### Function and Method Calls

You'll notice that the `.` operator relaxes those rules a bit.
In the method call `player.location()`, `player` might be a `Player`, a reference of type `&Player`, or a smart pointer of type `Box<Player>` or `Rc<Player>`.
The `.location()` method might take the `player` either by value or by reference.
The same `.location()` syntax works in all cases, because Rust's `.` operator automatically dereferences `player` or borrows a reference to it as needed.

### Fields and Elements

### Reference Operators

### Arithmetic, Bitwise, Comparison, and Logical Operators

### Assignment

### Type Casts

Converting a value from one type to another usually requires an explicit cast in Rust.
Casts use the `as` keyword:
```rust
let x = 17;              // x is type i32
let index = x as usize;  // convert to usize
```

### Closures

### Onward

## Chapter 7. Error Handling

Rust's approach to error handling is unusual enough to warrant a short chapter on the topic.
There aren't any difficult ideas here, just ideas that might be new to you.
This chapter covers the two different kinds of error handling in Rust: panic and `Result`s.

Ordinary errors are handled using the `Result` type.
`Result`s typically represent problems caused by things outside the program, like erroneous input, a network outage, or a permissions problem.
That such situations occur is not up to us; even a bug-free program will encounter them from time to time.
Most of this chapter is dedicated to that kind of error.
We'll cover panic first, though, because it's the simpler of the two.

Panic is for the other kind of error, the kind that *should never happen*.

### Panic

A program panics when it encounters something so messed up that there must be a bug in the program itself.
Something like:
* Out-of-bounds array access
* Integer division by zero
* Calling `.expect()` on a `Result` that happens to be `Err`
* Assertion failure

#### Unwinding

#### Aborting

Stack unwinding is the default panic behavior, but there are two circumstances in which Rust does not try to unwind the stack.

If a `.drop()` method triggers a second panic while Rust is still trying to clean up after the first, this is considered fatal.
Rust stops unwinding and aborts the whole process.

### Result

Rust doesn't have exceptions.
Instead, functions that can fail have a return type that says so:
```rust
fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error>
```
The `Result` type indicates possible failure.

#### Catching Errors

The most thorough way of dealing with a `Result` is the way we showed in Chapter 2: use a `match` expression.
```rust
match get_weather(hometown) {
    Ok(report) => {
        display_weather(hometown, &report);
    }
    Err(err) => {
        println!("error querying the weather: {}", err);
        schedule_weather_retry();
    }
}
```
This is Rust's equivalent of `try/catch` in other languages.
It's what you use when you want to handle errors head-on, not pass them on to your caller.

`match` is a bit verbose, so `Result<T, E>` offers a variety of methods that are useful in particular common cases.
Each of these methods has a `match` expression in its implementation.
(For the full list of `Result` methods, consult the online documentation. The methods listed here are the ones we use the most.)

#### Result Type Aliases

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

When something like `Result<()>` appears in the online documentation, you can click on the identifier `Result` to see which type alias is being used and learn the error type.
In practice, it's usually obvious from context.

#### Printing Errors

Sometimes the only way to handle an error is by dumping it to the terminal and moving on.
We already showed one way to do this:
```rust
println!("error querying the weather: {}", err);
```

#### Propagating Errors

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

#### Working with Multiple Error Types

Often, more than one thing could go wrong.
Suppose we are simply reading numbers from a text file:
```rust
use std::io::{self, BufRead};

/// Read integers from a text file.
/// The file should have one number on each line.
fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, io::Error> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;         // reading lines can fail
        numbers.push(line.parse()?);     // parsing integers can fail
    }
    Ok(numbers)
}
```

A simpler approach is to use what’s built into Rust.
All of the standard library error types can be converted to the type `Box<dyn std::error::Error + Send + Sync + 'static>`.
This is a bit of a mouthful, but `dyn std::error::Error` represents "any error," and `Send + Sync + 'static` makes it safe to pass between threads, which you'll often want.
For convenience, you can define type aliases:
```rust
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;
```

#### Dealing with Errors That "Can't Happen"

#### Ignoring Errors

#### Handling Errors in `main()`

In most places where a `Result` is produced, letting the error bubble up to the caller is the right behavior.
This is why `?` is a single character in Rust.
As we've seen, in some programs it's used on many lines of code in a row.

But if you propagate an error long enough, eventually it reaches `main()`, and something has to be done with it.
Normally, `main()` can't use `?` because its return type is not `Result`:
```rust
fn main() {
    calculate_tides()?;  // error: can't pass the buck any further
}
```
The simplest way to handle errors in `main()` is to use `.expect()`:
```rust
fn main() {
    calculate_tides().expect("error");  // the buck stops here
}
```
However, you can also change the type signature of `main()` to return a `Result` type, so you can use `?`:
```rust
fn main() -> Result<(), TideCalcError> {
    let tides = calculate_tides()?;
    print_tides(tides);
    Ok(())
}
```
If you have more complex error types or want to include more details in your message, it pays to print the error message yourself:

#### Declaring a Custom Error Type

#### Why Results?

## Chapter 8. Crates and Modules

This chapter covers the features of Rust that help keep your program organized: crates and modules.

### Crates

Rust programs are made of *crates*.
Each crate is a complete, cohesive unit: all the source code for a single library or executable, plus any associated tests, examples, tools, configuration, and other junk.

The easiest way to see what crates are and how they work together is to use cargo build with the `--verbose` flag to build an existing project that has some dependencies.

The word *dependencies* here just means other crates this project uses: code we're depending on.
We found these crates on [crates.io](https://crates.io/), the Rust community's site for open source crates.

The Cargo transcript tells the story of how this information is used.
When we run `cargo build`, Cargo starts by downloading source code for the specified versions of these crates from crates.io.
Then, it reads those crates' *Cargo.toml* files, downloads their dependencies, and so on recursively.

The collection of all these dependency relationships, which tells Cargo everything it needs to know about what crates to build and in what order, is known as the *dependency graph* of the crate.
Cargo's automatic handling of the dependency graph and transitive dependencies is a huge win in terms of programmer time and effort.

#### Editions

To evolve without breaking existing code, Rust uses *editions*.

#### Build Profiles

### Modules

Whereas crates are about code sharing between projects, *modules* are about code organization within a project.
They act as Rust's namespaces, containers for the functions, types, constants, and so on that make up your Rust program or library.

The `pub` keyword makes an item public, so it can be accessed from outside the module.

One function is marked `pub(crate)`, meaning that it is available anywhere inside this crate, but isn't exposed as part of the external interface.
It can't be used by other crates, and it won't show up in this crate's documentation.

Anything that isn't marked `pub` is private and can only be used in the same module in which it is defined, or any child modules:

#### Nested Modules

Modules can nest, and it's fairly common to see a module that's just a collection of submodules:
```rust
mod plant_structures {
    pub mod roots {
        ...
    }
    pub mod stems {
        ...
    }
    pub mod leaves {
        ...
    }
}
```

#### Modules in Separate Files

A module can also be written like this:
```rust
mod spores;
```
Earlier, we included the body of the `spores` module, wrapped in curly braces.
Here, we're instead telling the Rust compiler that the `spores` module lives in a separate file, called *spores.rs*:
```rust
// spores.rs

/// A cell made by an adult fern...
pub struct Spore {
    ...
}

/// Simulate the production of a spore by meiosis.
pub fn produce_spore(factory: &mut Sporangium) -> Spore {
    ...
}

/// Extract the genes in a particular spore.
pub(crate) fn genes(spore: &Spore) -> Vec<Gene> {
    ...
}

/// Mix genes to prepare for meiosis (part of interphase).
fn recombine(parent: &mut Cell) {
    ...
}
```
*spores.rs* contains only the items that make up the module.
It doesn't need any kind of boilerplate to declare that it's a module.

#### Paths and Imports

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

#### The Standard Prelude

#### Making `use` Declarations `pub`

#### Making Struct Fields `pub`

#### Statics and Constants

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

The definition of a named-field struct type looks like this:
```rust
/// A rectangle of eight-bit grayscale pixels.
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize)
}
```
The convention in Rust is for all types, structs included, to have names that capitalize the first letter of each word, like `GrayscaleMap`, a convention called *CamelCase* (or *PascalCase*).
Fields and methods are lowercase, with words separated by underscores. This is called *snake_case*.

You can construct a value of this type with a *struct expression*, like this:
```rust
let width = 1024;
let height = 576;
let image = GrayscaleMap {
    pixels: vec![0; width * height],
    size: (width, height)
};
```

To access a struct's fields, use the familiar `.` operator:
```rust
assert_eq!(image.size, (1024, 576));
assert_eq!(image.pixels.len(), 1024 * 576);
```

Like all other items, structs are private by default, visible only in the module where they're declared and its submodules.
You can make a struct visible outside its module by prefixing its definition with `pub`.
The same goes for each of its fields, which are also private by default:
```rust
/// A rectangle of eight-bit grayscale pixels.
pub struct GrayscaleMap {
    pub pixels: Vec<u8>,
    pub size: (usize, usize)
}
```

### Tuple-Like Structs

The second kind of struct type is called a *tuple-like* struct, because it resembles a tuple:
```rust
struct Bounds(usize, usize);
```
You construct a value of this type much as you would construct a tuple, except that you must include the struct name:
```rust
let image_bounds = Bounds(1024, 768);
```
Tuple-like structs are good for *newtypes*, structs with a single component that you define to get stricter type checking.
For example, if you are working with ASCII-only text, you might define a newtype like this:
```rust
struct Ascii(Vec<u8>);
```
Using this type for your ASCII strings is much better than simply passing around `Vec<u8>` buffers and explaining what they are in the comments.
The newtype helps Rust catch mistakes where some other byte buffer is passed to a function expecting ASCII text.


### Unit-Like Structs

### Struct Layout

### Defining Methods with `impl`

You can define methods on your own struct types as well.
Rather than appearing inside the struct definition, as in C++ or Java, Rust methods appear in a separate `impl` block.

An `impl` block is simply a collection of fn definitions, each of which becomes a method on the struct type named at the top of the block.

Functions defined in an `impl` block are called *associated functions*, since they're associated with a specific type.
The opposite of an associated function is a *free function*, one that is not defined as an `impl` block's item.

Rust passes a method the value it's being called on as its first argument, which must have the special name `self`.
Since `self`'s type is obviously the one named at the top of the `impl` block, or a reference to that, Rust lets you omit the type, and write `self`, `&self`, or `&mut self` as shorthand for `self: Queue`, `self: &Queue`, or `self: &mut Queue`.

Unlike C++ and Java, where the members of the "this" object are directly visible in method bodies as unqualified identifiers, a Rust method must explicitly use `self` to refer to the value it was called on, similar to the way Python methods use `self`, and the way JavaScript methods use `this`.

#### Passing Self as a Box, Rc, or Arc

#### Type-Associated Functions

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

### Associated Consts

### Generic Structs

Fortunately, Rust structs can be *generic*, meaning that their definition is a template into which you can plug whatever types you like.

In generic struct definitions, the type names used in <angle brackets> are called *type parameters*.

### Generic Structs with Lifetime Parameters

### Generic Structs with Constant Parameters

### Deriving Common Traits for Struct Types

But in the case of these standard traits, and several others, you don't need to implement them by hand unless you want some kind of custom behavior. Rust can automatically implement them for you, with mechanical accuracy.
Just add a `#[derive]` attribute to the struct:
```rust
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64
}
```
Each of these traits can be implemented automatically for a struct, provided that each of its fields implements the trait.
We can ask Rust to derive `PartialEq` for `Point` because its two fields are both of type `f64`, which already implements `PartialEq`.

### Interior Mutability

## Chapter 10. Enums and Patterns

A Rust enum can also contain data, even data of varying types.

### Enums

Simple, C-style enums are straightforward:
```rust
enum Ordering {
    Less,
    Equal,
    Greater,
}
```
This declares a type `Ordering` with three possible values, called *variants* or *constructors*: `Ordering::Less`, `Ordering::Equal`, and `Ordering::Greater`.

In memory, values of C-style enums are stored as integers.

Casting a C-style enum to an integer is allowed:
```rust
assert_eq!(HttpStatus::Ok as i32, 200);
```
However, casting in the other direction, from the integer to the enum, is not.
Unlike C and C++, Rust guarantees that an enum value is only ever one of the values spelled out in the enum declaration.
An unchecked cast from an integer type to an enum type could break this guarantee, so it's not allowed.

Enums can have methods, just like structs:
```rust
impl TimeUnit {
    /// Return the plural noun for this time unit.
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    /// Return the singular noun for this time unit.
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}
```
So much for C-style enums.
The more interesting sort of Rust enum is the kind whose variants hold data.

### Enums with Data

In all, Rust has three kinds of enum variant, echoing the three kinds of struct we showed in the previous chapter.
Variants with no data correspond to unit-like structs.
Tuple variants look and function just like tuple structs.
Struct variants have curly braces and named fields.
A single enum can have variants of all three kinds:
```rust
enum RelationshipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        car: DifferentialEquation,
        cdr: EarlyModernistPoem,
    },
}
```
All constructors and fields of an enum share the same visibility as the enum itself.

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

*Generics* are the other flavor of polymorphism in Rust.

### Using Traits

A trait is a feature that any given type may or may not support.
Most often, a trait represents a capability: something a type can do.
* A value that implements `std::io::Write` can write out bytes.
* A value that implements `std::iter::Iterator` can produce a sequence of values.
* A value that implements `std::clone::Clone` can make clones of itself in memory.
* A value that implements `std::fmt::Debug` can be printed using `println!()` with the `{:?}` format specifier.

Only calls through `&mut dyn Write` incur the overhead of a dynamic dispatch, also known as a virtual method call, which is indicated by the `dyn` keyword in the type.
`dyn Write` is known as a *trait object*; we'll look at the technical details of trait objects, and how they compare to generic functions, in the following sections.

#### Trait Objects

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
Each trait object therefore takes up two machine words, as shown in Figure 11-1.

In Rust, as in C++, the vtable is generated once, at compile time, and shared by all objects of the same type.

Rust automatically converts ordinary references into trait objects when needed.
This is why we're able to pass `&mut local_file` to `say_hello` in this example:
```rust
let mut local_file = File::create("hello.txt")?;
say_hello(&mut local_file)?;
```

### Defining and Implementing Traits

Defining a trait is simple.
Give it a name and list the type signatures of the trait methods.

To implement a trait, use the syntax `impl TraitName for Type`:

#### Default Methods

#### Traits and Other People's Types

Rust lets you implement any trait on any type, as long as either the trait or the type is introduced in the current crate.

#### `Self` in Traits

A trait can use the keyword `Self` as a type.

#### Subtraits

We can declare that a trait is an extension of another trait:
```rust
/// Someone in the game world, either the player or some other
/// pixie, gargoyle, squirrel, ogre, etc.
trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> Direction;
    ...
}
```
The phrase trait `Creature: Visible` means that all creatures are visible.
Every type that implements `Creature` must also implement the `Visible` trait:

In fact, Rust's subtraits are really just a shorthand for a bound on `Self`.
A definition of `Creature` like this is exactly equivalent to the one shown earlier:
```rust
trait Creature where Self: Visible {
    ...
}
```

#### Type-Associated Functions

In most object-oriented languages, interfaces can't include static methods or constructors, but traits can include type-associated functions, Rust's analog to static methods:
```rust
trait StringSet {
    /// Return a new empty set.
    fn new() -> Self;

    /// Return a set that contains all the strings in `strings`.
    fn from_slice(strings: &[&str]) -> Self;

    /// Find out if this set contains a particular `value`.
    fn contains(&self, string: &str) -> bool;

    /// Add a string to this set.
    fn add(&mut self, string: &str);
}
```

### Fully Qualified Method Calls

All the ways for calling trait methods we've seen so far rely on Rust filling in some missing pieces for you.
For example, suppose you write the following:
```rust
"hello".to_string()
```
It's understood that `to_string` refers to the `to_string` method of the `ToString` trait, of which we're calling the `str` type's implementation.
So there are four players in this game: the trait, the method of that trait, the implementation of that method, and the value to which that implementation is being applied.
It's great that we don't have to spell all that out every time we want to call a method.
But in some cases you need a way to say exactly what you mean.
Fully qualified method calls fit the bill.

First of all, it helps to know that a method is just a special kind of function.
These two calls are equivalent:
```rust
"hello".to_string()

str::to_string("hello")
```
The second form looks exactly like a associated function call.
This works even though the `to_string` method takes a `self` argument.
Simply pass `self` as the function's first argument.

### Traits That Define Relationships Between Types

#### Associated Types (or How Iterators Work)

#### Generic Traits (or How Operator Overloading Works)

#### `impl` Trait

Rust has a feature called `impl Trait` designed for precisely this situation.
`impl Trait` allows us to "erase" the type of a return value, specifying only the trait or traits it implements, without dynamic dispatch or a heap allocation:
```rust
fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item=u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}
```

## Chapter 12. Operator Overloading

You can make your own types support arithmetic and other operators, too, just by implementing a few built-in traits.
This is called *operator overloading*, and the effect is much like operator overloading in C++, C#, Python, and Ruby.

### Arithmetic and Bitwise Operators

In Rust, the expression `a + b` is actually shorthand for `a.add(b)`, a call to the `add` method of the standard library's `std::ops::Add` trait.
Rust's standard numeric types all implement `std::ops::Add`.

## Unary Operators

## Binary Operators

## Compound Assignment Operators

## Equivalence Comparisons

Rust's equality operators, `==` and `!=`, are shorthand for calls to the `std::cmp::PartialEq` trait's `eq` and `ne` methods:
```rust
assert_eq!(x == y, x.eq(&y));
assert_eq!(x != y, x.ne(&y));
```
Here's the definition of `std::cmp::PartialEq`:
```rust
trait PartialEq<Rhs = Self>
where
    Rhs: ?Sized,
{
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}
```
Since the `ne` method has a default definition, you only need to define `eq` to implement the `PartialEq` trait, so here's a complete implementation for `Complex`:
```rust
impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}
```
In other words, for any component type `T` that itself can be compared for equality, this implements comparison for `Complex<T>`.

Implementations of `PartialEq` are almost always of the form shown here: they compare each field of the left operand to the corresponding field of the right.
These get tedious to write, and equality is a common operation to support, so if you ask, Rust will generate an implementation of `PartialEq` for you automatically.
Simply add `PartialEq` to the type definition's `derive` attribute like so:
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
struct Complex<T> {
    ...
}
```
Rust's automatically generated implementation is essentially identical to our hand-written code, comparing each field or element of the type in turn.
Rust can derive `PartialEq` implementations for enum types as well.
Naturally, each of the values the type holds (or might hold, in the case of an enum) must itself implement `PartialEq`.

## Chapter 13. Utility Traits

### `Drop`

When a value's owner goes away, we say that Rust *drops* the value.
Dropping a value entails freeing whatever other values, heap storage, and system resources the value owns.
Drops occur under a variety of circumstances: when a variable goes out of scope; at the end of an expression statement; when you truncate a vector, removing elements from its end; and so on.

### `Sized`

A *sized type* is one whose values all have the same size in memory.

All sized types implement the `std::marker::Sized` trait, which has no methods or associated types.
Rust implements it automatically for all types to which it applies; you can't implement it yourself.
The only use for `Sized` is as a bound for type variables: a bound like `T: Sized` requires `T` to be a type whose size is known at compile time.
Traits of this sort are called *marker traits*, because the Rust language itself uses them to mark certain types as having characteristics of interest.

However, Rust also has a few *unsized types* whose values are not all the same size.
For example, the string slice type `str` (note, without an `&`) is unsized.

Since unsized types are so limited, most generic type variables should be restricted to `Sized` types.
In fact, this is necessary so often that it is the implicit default in Rust: if you write `struct S<T> { ... }`, Rust understands you to mean struct `S<T: Sized> { ... }`.
If you do not want to constrain `T` this way, you must explicitly opt out, writing struct `S<T: ?Sized> { ... }`.
The `?Sized` syntax is specific to this case and means "not necessarily `Sized`."
For example, if you write `struct S<T: ?Sized> { b: Box<T> }`, then Rust will allow you to write `S<str>` and `S<dyn Write>`, where the box becomes a fat pointer, as well as `S<i32>` and `S<String>`, where the box is an ordinary pointer.

### `Clone`

### `Copy`

### `Deref` and `DerefMut`

You can specify how dereferencing operators like `*` and `.` behave on your types by implementing the `std::ops::Deref` and `std::ops::DerefMut` traits.
Pointer types like `Box<T>` and `Rc<T>` implement these traits so that they can behave as Rust's built-in pointer types do.
For example, if you have a `Box<Complex>` value `b`, then `*b` refers to the `Complex` value that `b` points to, and `b.re` refers to its real component.
If the context assigns or borrows a mutable reference to the referent, Rust uses the `DerefMut` ("dereference mutably") trait; otherwise, read-only access is enough, and it uses `Deref`.

The traits are defined like this:
```rust
trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}

trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
```
The `deref` and `deref_mut` methods take a `&Self` reference and return a `&Self::Target` reference.
`Target` should be something that `Self` contains, owns, or refers to: for `Box<Complex>` the `Target` type is `Complex`.
Note that `DerefMut` extends `Deref`: if you can dereference something and modify it, certainly you should be able to borrow a shared reference to it as well.
Since the methods return a reference with the same lifetime as `&self`, `self` remains borrowed for as long as the returned reference lives.

The `Deref` and `DerefMut` traits play another role as well.
Since `deref` takes a `&Self` reference and returns a `&Self::Target` reference, Rust uses this to automatically convert references of the former type into the latter.
In other words, if inserting a `deref` call would prevent a type mismatch, Rust inserts one for you.
Implementing `DerefMut` enables the corresponding conversion for mutable references.
These are called the *deref coercions*: one type is being "coerced" into behaving as another.

Rust will apply several deref coercions in succession if necessary.

### `Default`

### `AsRef` and `AsMut`

When a type implements `AsRef<T>`, that means you can borrow a `&T` from it efficiently.
`AsMut` is the analogue for mutable references.
Their definitions are as follows:
```rust
trait AsRef<T: ?Sized> {
    fn as_ref(&self) -> &T;
}

trait AsMut<T: ?Sized> {
    fn as_mut(&mut self) -> &mut T;
}
```

`AsRef` is typically used to make functions more flexible in the argument types they accept.
For example, the `std::fs::File::open` function is declared like this:
```rust
fn open<P: AsRef<Path>>(path: P) -> Result<File>
```

### `Borrow` and `BorrowMut`

### `From` and `Into`

The `std::convert::From` and `std::convert::Into` traits represent conversions that consume a value of one type and return a value of another.
Whereas the `AsRef` and `AsMut` traits borrow a reference of one type from another, `From` and `Into` take ownership of their argument, transform it, and then return ownership of the result back to the caller.

Their definitions are nicely symmetrical:
```rust
trait Into<T>: Sized {
    fn into(self) -> T;
}

trait From<T>: Sized {
    fn from(other: T) -> Self;
}
```
The standard library automatically implements the trivial conversion from each type to itself: every type `T` implements `From<T>` and `Into<T>`.



## Chapter 14. Closures

### Capturing Variables

A closure can use data that belongs to an enclosing function.
For example:
```rust
/// Sort by any of several different statistics.
fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) {
    cities.sort_by_key(|city| -city.get_statistic(stat));
}
```
The closure here uses `stat`, which is owned by the enclosing function, `sort_by_statistic`.
We say that the closure "captures" `stat`.
This is one of the classic features of closures, so naturally, Rust supports it; but in Rust, this feature comes with a string attached.

#### Closures That Borrow

#### Closures That Steal

### Function and Closure Types

Throughout this chapter, we've seen functions and closures used as values.
Naturally, this means that they have types.
For example:
```rust
fn city_population_descending(city: &City) -> i64 {
    -city.population
}
```
This function takes one argument (a `&City`) and returns an `i64`.
It has the type `fn(&City) -> i64`.

## Chapter 15. Iterators

An *iterator* is a value that produces a sequence of values, typically for a loop to operate on.
Rust's standard library provides iterators that traverse vectors, strings, hash tables, and other collections, but also iterators to produce lines of text from an input stream, connections arriving at a network server, values received from other threads over a communications channel, and so on.
And of course, you can implement iterators for your own purposes.
Rust's for loop provides a natural syntax for using iterators, but iterators themselves also provide a rich set of methods for mapping, filtering, joining, collecting, and so on.

### The `Iterator` and `IntoIterator` Traits

An iterator is any value that implements the `std::iter::Iterator` trait:
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    ... // many default methods
}
```

Under the hood, every `for` loop is just shorthand for calls to `IntoIterator` and `Iterator` methods:
```rust
let mut iterator = (&v).into_iter();
while let Some(element) = iterator.next() {
    println!("{}", element);
}
```

### Creating Iterators

The Rust standard library documentation explains in detail what sort of iterators each type provides, but the library follows some general conventions to help you get oriented and find what you need.

#### `iter` and `iter_mut` Methods

#### `IntoIterator` Implementations

Most collections actually provide several implementations of `IntoIterator`, for shared references (`&T`), mutable references (`&mut T`), and moves (`T`):

### Iterator Adapters

Once you have an iterator in hand, the `Iterator` trait provides a broad selection of *adapter methods*, or simply *adapters*, that consume one iterator and build a new one with useful behaviors.
To see how adapters work, we'll start with two of the most popular adapters, `map` and `filter`.
Then we'll cover the rest of the adapter toolbox, covering almost any way you can imagine to make sequences of values from other sequences: truncation, skipping, combination, reversal, concatenation, repetition, and more.

#### `map` and `filter`

The `Iterator` trait's `map` adapter lets you transform an iterator by applying a closure to its items.
The `filter` adapter lets you filter out items from an iterator, using a closure to decide which to keep and which to drop.

There are two important points to notice about iterator adapters.

First, simply calling an adapter on an iterator doesn't consume any items;
it just returns a new iterator, ready to produce its own items by drawing from the first iterator as needed.
In a chain of adapters, the only way to make any work actually get done is to call `next` on the final iterator.

### Consuming Iterators

#### Building Collections: `collect` and `FromIterator`

Throughout the book, we've been using the `collect` method to build vectors holding an iterator's items.
For example, in Chapter 2, we called `std::env::args()` to get an iterator over the program's command-line arguments and then called that iterator's `collect` method to gather them into a vector:
```rust
let args: Vec<String> = std::env::args().collect();
```

Naturally, `collect` itself doesn't know how to construct all these types.
Rather, when some collection type like `Vec` or `HashMap` knows how to construct itself from an iterator, it implements the `std::iter::FromIterator` trait, for which `collect` is just a convenient veneer:
```rust
trait FromIterator<A>: Sized {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self;
}
```
If a collection type implements `FromIterator<A>`, then its type-associated function `from_iter` builds a value of that type from an iterable producing items of type `A`.

#### The `Extend` Trait

#### `partition`

#### `for_each` and `try_for_each`

## Chapter 16. Collections

## Chapter 17. Strings and Text

## Chapter 18. Input and Output

### Readers and Writers

#### Readers

#### Buffered Readers

For efficiency, readers and writers can be *buffered*, which simply means they have a chunk of memory (a buffer) that holds some input or output data in memory.

### Files and Directories

## 19. Concurrency

### Fork-Join Parallelism

### Channels

A *channel* is a one-way conduit for sending values from one thread to another.
In other words, it's a thread-safe queue.

#### Sending Values

## Chapter 20. Asynchronous Programming

Threads are good and necessary for distributing work across multiple processors, but their memory demands are such that we often need complementary ways, used together with threads, to break the work down.

You can use Rust *asynchronous tasks* to interleave many independent activities on a single thread or a pool of worker threads.
Asynchronous tasks are similar to threads, but are much quicker to create, pass control amongst themselves more efficiently, and have memory overhead an order of magnitude less than that of a thread.
It is perfectly feasible to have hundreds of thousands of asynchronous tasks running simultaneously in a single program.
Of course, your application may still be limited by other factors like network bandwidth, database speed, computation, or the work's inherent memory requirements, but the memory overhead inherent in the use of tasks is much less significant than that of threads.

Generally, asynchronous Rust code looks very much like ordinary multithreaded code, except that operations that might block, like I/O or acquiring mutexes, need to be handled a bit differently.
Treating these specially gives Rust more information about how your code will behave, which is what makes the improved performance possible.

### From Synchronous to Asynchronous

While this function is waiting for the system calls to return, its single thread is blocked: it can't do anything else until the system call finishes.

To get around this, a thread needs to be able to take up other work while it waits for system calls to complete.
But it's not obvious how to accomplish this.
For example, the signature of the function we're using to read the response from the socket is:
```rust
fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize>;
```
It's written right into the type: this function doesn't return until the job is done, or something goes wrong.
This function is *synchronous*: the caller resumes when the operation is complete.
If we want to use our thread for other things while the operating system does its work, we're going need a new I/O library that provides an *asynchronous* version of this function.

#### Futures

Rust's approach to supporting asynchronous operations is to introduce a trait, `std::future::Future`:
```rust
trait Future {
    type Output;
    // For now, read `Pin<&mut Self>` as `&mut Self`.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```
A `Future` represents an operation that you can test for completion.
A future's `poll` method never waits for the operation to finish: it always returns immediately.
If the operation is complete, `poll` returns `Poll::Ready(output)`, where `output` is its final result.
Otherwise, it returns `Pending`.
If and when the future is worth polling again, it promises to let us know by invoking a *waker*, a callback function supplied in the `Context`.
We call this the "piñata model" of asynchronous programming: the only thing you can do with a future is whack it with a `poll` until a value falls out.

This is the general pattern: the asynchronous version of any function takes the same arguments as the synchronous version, but the return type has a `Future` wrapped around it.

#### Async Functions and Await Expressions

Here's a version of `cheapo_request` written as an *asynchronous function*:
```rust
use async_std::io::prelude::*;
use async_std::net;

async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String>
{
    let mut socket = net::TcpStream::connect((host, port)).await?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}
```
This is token for token the same as our original version, except:
* The function starts with `async fn` instead of `fn`.
* It uses the `async_std` crate's asynchronous versions of `TcpStream::connect`, `write_all`, and `read_to_string`.
These all return futures of their results.
(The examples in this section use version `1.7` of `async_std`.)
* After each call that returns a future, the code says `.await`.
Although this looks like a reference to a struct field named `await`, it is actually special syntax built into the language for waiting until a future is ready.
An `await` expression evaluates to the final value of the future.
This is how the function obtains the results from `connect`, `write_all`, and `read_to_string`.

Unlike an ordinary function, when you call an asynchronous function, it returns immediately, before the body begins execution at all.
Obviously, the call's final return value hasn't been computed yet; what you get is a *future of* its final value.
So if you execute this code:
```rust
let response = cheapo_request(host, port, path);
```
then `response` will be a future of a `std::io::Result<String>`, and the body of `cheapo_request` has not yet begun execution.
You don't need to adjust an asynchronous function's return type;
Rust automatically treats `async fn f(...) -> T` as a function that returns a future of a `T`, not a `T` directly.

The future returned by an async function wraps up all the information the function body will need to run: the function's arguments, space for its local variables, and so on.
(It's as if you'd captured the call's stack frame as an ordinary Rust value.)
So `response` must hold the values passed for `host`, `port`, and `path`, since `cheapo_request`'s body is going to need those to run.

The future's specific type is generated automatically by the compiler, based on the function's body and arguments.
This type doesn't have a name;
all you know about it is that it implements `Future<Output=R>`, where `R` is the async function's return type.
In this sense, futures of asynchronous functions are like closures: closures also have anonymous types, generated by the compiler, that implement the `FnOnce`, `Fn`, and `FnMut` traits.

When you first poll the future returned by `cheapo_request`, execution begins at the top of the function body and runs until the first `await` of the future returned by `TcpStream::connect`.
The `await` expression polls the `connect` future, and if it is not ready, then it returns `Poll::Pending` to its own caller: polling `cheapo_request`'s future cannot proceed past that first `await` until a poll of `TcpStream::connect`'s future returns `Poll::Ready`.
So a rough equivalent of the expression `TcpStream::connect(...).await` might be:
```rust
{
    // Note: this is pseudocode, not valid Rust
    let connect_future = TcpStream::connect(...);
    'retry_point:
    match connect_future.poll(cx) {
        Poll::Ready(value) => value,
        Poll::Pending => {
            // Arrange for the next `poll` of `cheapo_request`'s
            // future to resume execution at 'retry_point.
            ...
            return Poll::Pending;
        }
    }
}
```
An `await` expression takes ownership of the future and then polls it.
If it's ready, then the future's final value is the value of the `await` expression, and execution continues.
Otherwise, it returns the `Poll::Pending` to its own caller.

But crucially, the next poll of `cheapo_request`'s future doesn't start at the top of the function again: instead, it resumes execution mid-function at the point where it is about to poll `connect_future`.
We don't progress to the rest of the async function until that future is ready.

As `cheapo_request`'s future continues to be polled, it will work its way through the function body from one `await` to the next, moving on only when the subfuture it's awaiting is ready.
Thus, how many times `cheapo_request`'s future must be polled depends on both the behavior of the subfutures and the function's own control flow.
`cheapo_request`'s future tracks the point at which the next poll should resume, and all the local state - variables, arguments, temporaries — that resumption will need.

The ability to suspend execution mid-function and then resume later is unique to async functions.
When an ordinary function returns, its stack frame is gone for good.
Since `await` expressions depend on the ability to resume, you can only use them inside async functions.

As of this writing, Rust does not yet allow traits to have asynchronous methods.
Only free functions and functions inherent to a specific type can be asynchronous.
Lifting this restriction will require a number of changes to the language.
In the meantime, if you need to define traits that include async functions, consider using the `async_trait` crate, which provides a macro-based workaround.

#### Calling Async Functions from Synchronous Code: `block_on`

In a sense, async functions just pass the buck.
True, it's easy to get a future's value in an async function: just `await` it.
But the async function *itself* returns a future, so it's now the caller's job to do the polling somehow.
Ultimately, someone has to actually wait for a value.

We can call `cheapo_request` from an ordinary, synchronous function (like `main`, for example) using `async_std`'s `task::block_on` function, which takes a future and polls it until it produces a value:
```rust
fn main() -> std::io::Result<()> {
    use async_std::task;

    let response = task::block_on(cheapo_request("example.com", 80, "/"))?;
    println!("{}", response);
    Ok(())
}
```
Since `block_on` is a synchronous function that produces the final value of an asynchronous function, you can think of it as an adapter from the asynchronous world to the synchronous world.
But its blocking character also means that you should never use `block_on` within an async function: it would block the entire thread until the value is ready.
Use `await` instead.

It doesn't sound too hard to just write a loop that calls `poll` over and over.
But what makes `async_std::task::block_on` valuable is that it knows how to go to sleep until the future is actually worth polling again, rather than wasting your processor time and battery life making billions of fruitless `poll` calls.
The futures returned by basic I/O functions like `connect` and `read_to_string` retain the waker supplied by the `Context` passed to `poll` and invoke it when `block_on` should wake up and try polling again.

This is a lot of detail.
Fortunately, you can usually just think in terms of the simplified upper timeline: some function calls are sync, others are async and need an `await`, but they're all just function calls.
The success of Rust's asynchronous support depends on helping programmers work with the simplified view in practice, without being distracted by the back-and-forth of the implementation.

#### Spawning Async Tasks

The `async_std::task::block_on` function blocks until a future's value is ready.
But blocking a thread completely on a single future is no better than a synchronous call: the goal of this chapter is to get the thread *doing other work* while it's waiting.

For this, you can use `async_std::task::spawn_local`.
This function takes a future and adds it to a pool that `block_on` will try polling whenever the future it's blocking on isn't ready.
So if you pass a bunch of futures to `spawn_local` and then apply `block_on` to a future of your final result, `block_on` will poll each spawned future whenever it is able to make progress, running the entire pool concurrently until your result is ready.

#### Async Blocks

In addition to asynchronous functions, Rust also supports *asynchronous blocks*.
Whereas an ordinary block statement returns the value of its last expression, an async block returns *a future of* the value of its last expression.
You can use `await` expressions within an async block.

An async block looks like an ordinary block statement, preceded by the `async` keyword:
```rust
let serve_one = async {
    use async_std::net;

    // Listen for connections, and accept one.
    let listener = net::TcpListener::bind("localhost:8087").await?;
    let (mut socket, _addr) = listener.accept().await?;

    // Talk to client on `socket`.
    ...
};
```

#### Building Async Functions from Async Blocks

Asynchronous blocks give us another way to get the same effect as an asynchronous function, with a little more flexibility.

####

#### But Does Your Future Implement `Send`?

#### Long Running Computations: `yield_now` and `spawn_blocking`

#### Comparing Asynchronous Designs

In many ways Rust's approach to asynchronous programming resembles that taken by other languages.
For example, JavaScript, C#, and Rust all have asynchronous functions with await expressions.
And all these languages have values that represent incomplete computations: Rust calls them "futures," JavaScript calls them "promises," and C# calls them "tasks," but they all represent a value that you may have to wait for.

Rust's use of polling, however, is unusual.
In JavaScript and C#, an asynchronous function begins running as soon as it is called, and there is a global event loop built into the system library that resumes suspended async function calls when the values they were awaiting become available.
In Rust, however, an async call does nothing until you pass it to a function like `block_on`, `spawn`, or `spawn_local` that will poll it and drive the work to completion.
These functions, called *executors*, play the role that other languages cover with a global event loop.

## Chapter 21. Macros

Rust supports *macros*, a way to extend the language in ways that go beyond what you can do with functions alone.

Macros are a kind of shorthand.
During compilation, before types are checked and long before any machine code is generated, each macro call is *expanded* - that is, it's replaced with some Rust code.

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

#### Unintended Consequences


