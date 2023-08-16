# Rust Web Development

## 1 Why Rust?

## 2 Laying the foundation

### 2.1 The Rust Playbook

#### 2.1.1 Model your resources with structs

#### 2.2.2 Rust's asynchronous environment

#### 2.2.3 Rust's handling of async/await

The intent of this syntax is to make writing asynchronous Rust code feel like synchronous, blocking code to the programmer.
In the background, Rust is transforming this piece of code into a state machine where each different await represents a state.
Once all of the states are ready, the function continues to the last line and returns the result.

#### 2.2.4 Rust's `Future` type

#### 2.2.5 Choosing a Runtime

Many criticize Rust for not shipping with a runtime, since it's just such a center piece in every web service.
On the other side, being able to choose a specific runtime for your needs has the advantage of tailoring your application to your performance and platform needs.

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
