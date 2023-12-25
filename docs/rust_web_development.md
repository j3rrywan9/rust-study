# Rust Web Development

## 1 Why Rust?

## 2 Laying the foundation

### 2.1 The Rust Playbook

#### 2.1.1 Model your resources with structs

### 2.2 Creating our web server

Our web framework dictates a lot behind the scenes: the runtime, the abstraction over HTTP (plus the HTTP server implementation), and the design around how to pass requests to route handlers.
You should therefore feel comfortable with the design decisions that a particular web framework took, and which runtime it is choosing.

#### 2.2.1 Handling multiple requests at once

#### 2.2.2 Rust's asynchronous environment

For Rust to have a smaller footprint, its developers decided not to include any runtime nor abstraction over the kernel async API.
This gives the programmer the chance to choose a runtime that fits the needs of a project.
This also future-proofs the language in case huge advances in runtimes occur further down the road.

#### 2.2.3 Rust's handling of async/await

The intent of this syntax is to make writing asynchronous Rust code feel like synchronous, blocking code to the programmer.
In the background, Rust is transforming this piece of code into a state machine where each different await represents a state.
Once all of the states are ready, the function continues to the last line and returns the result.

#### 2.2.4 Rust's `Future` type

#### 2.2.5 Choosing a Runtime

The *runtime* is at the center of your asynchronous web service.
Its design and performance play a major part in the underlying performance and security of your application.
In Node.js, you have the Google V8 engine to handle the task.
Go has its own runtime, which is developed by Google as well.

You don't need to know in detail how a runtime works and how it is executing your asynchronous code.
However, it is good to at least have heard the term and the concepts surrounding it.
You might run into problems later in your code, and knowledge about how your chosen runtime works could help you solve problems or rewrite your code.

Many criticize Rust for not shipping with a runtime, since it's just such a centerpiece in every web service.
On the other side, being able to choose a specific runtime for your needs has the advantage of tailoring your application to your performance and platform needs.

One of the most popular runtimes, called Tokio, is widely used throughout the industry.
It is therefore a first safe bet for your application.
We will choose Tokio for our example and later go into detail about how to choose a runtime for your needs.

The runtime is responsible for creating threads, polling our futures, and driving them to completion.
It is also responsible for passing on work to the kernel and making sure to use the asynchronous kernel API to not have any bottlenecks there as well.
Tokio uses a crate called [Mio](https://github.com/tokio-rs/mio) for the asynchronous communication with the operating system kernel.
You as a developer will probably never touch the code in Mio, but it is good to know which types of crates and abstraction layers you are pulling into your project when developing web servers with Rust.

#### 2.2.6 Choosing a web framework

With Rust being still a new playground for web services, you might need more active help from the dev team and the community to solve issues you might have along the way.

## 3 Create your first route handler

### 3.1 Getting to know our web framework - warp

#### 3.1.1 What is included in warp

Remember that Rust doesn't include an HTTP implementation in the standard library, therefore a web framework needs to either create its own HTTP protocol implementation or use a crate.
Warp is using a crate called hyper for this.
Hyper is a HTTP server written in Rust, which supports HTTP/1, HTTP/2 and asynchronous concepts, which makes it a perfect foundation for a web framework.

#### 3.1.2 Warp's Filter System

### 3.2 GET your first JSON response

Our web framework will bring its own HTTP server, and we have to bring in the runtime it is built on top, which is `tokio`.
Every time a HTTP request comes in, the framework is processing it in a few steps:
* Check the request path inside the HTTP request
* Check the HTTP request type (GET, PUT, POST etc.)
* Forward the request to a route handler which is responsible for the path and type
* Before forwarding it to the end route handler, the request can be passed through a so-called middleware which checks things like authentication headers or adds further information to the request object for the end route handler

No matter which framework you end up using, all of them follow the same design principles.
The actual implementation and how and when you call certain parts might differ though.

#### 3.2.1 Align with your framework's way of thinking

Your first step into implementing your API with a framework is to set up the smallest possible working version.
Afterwards, you implement the simplest path to see how your framework of choice generally behaves and wants things to be.

#### 3.2.2 Handling the success route

You can compose different filters with the `and()` keyword.
We start with the `get()` filter, and add the parts of the paths with an `and()`.
Each request will go through each filter, and if applicable, will call the `.and_then()` part and calls the method `get_questions`.

#### 3.2.3 Getting help from `serde`

The `serde` library bundles serialization and deserialization methods into one framework.
It is basically the standard serialization (and deserialization) framework in the Rust ecosystem.
It can transform structs to formats like JSON, TOML or BSON (and transform them back as well).
