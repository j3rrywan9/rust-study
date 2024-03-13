# [Decrusting the `axum` crate](https://www.youtube.com/watch?v=Wnb_n5YktO8)

## `axum`

`axum` is one that is a little bit different in the sense that it only really provides routing and request handlers.

And then all the middleware stuff, anything you might wanna do in between a request comes in and a response comes back apart from the actual handler function, all of that stuff is handled through the tower crate.
In particular, the [`tower::Service`](https://docs.rs/tower/latest/tower/trait.Service.html) trait.
We'll get into what exactly that means, but the essence of this is that `axum` is fairly small.
Instead, you get that via these other middlewares that are built around the same trait, this `tower::Service` trait.

But sort of from a 10,000-foot view, it is really just a way to express application routes.
So things like, `GET` to `/` and what should happen when a request comes in like that.
And then in addition, a sort of mechanism for writing those handlers that receive the requests and extracting interesting information in those handlers.

It has code that defines the routes that you want to accept.
It sets up a web server.
It defines handlers for each of them.
So that's the basic functionality of the crate.

It uses `hyper` under the hood.
So `hyper` takes care of all the HTTP parts, or at least all of the sort of protocol level HTTP parts.
And then, `axum` builds on top of `hyper`.

So one of the things that's interesting about axum's approach to the whole web framework thing is they lean a lot on the rest of the ecosystem.
They use `hyper` for handling all the HTTP bits.
They use `tower` for handling everything that has to do with middleware and the like.
They use `tracing` for anything that has to do with logs.
They use `matchit` crate for doing rout matching in an efficient way.
So there's a bunch of bits in there in `axum` that are really just intended to be the glue between all of these to provide you the framework you need to write HTTP services.

And a router is the sort of primary entry point in your application.
It is the thing that when a request comes in, the router determines what function ultimately gets called to handle that request.
A router has a bunch of routes.

### Module [`axum::routing`](https://docs.rs/axum/0.6.19/axum/routing/index.html)

So all of the sort of HTTP verbs are treated as separate method routers here.
And in addition, you can just create yourself.
So you can create one of these `MethodFilter`s that allows you to say, you know, `GET` and `POST`, for example, all get passed in here.

```rust
async fn create_user(
    Json<payload>: Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    ...
}
```
You might think this looks weird.
Like why is there something on the left of the colon?
This is just structure pattern matching.
If we look at the `Json` type, it is just a tuple struct with a single field in it.
```rust
pub struct Json<T>(pub T);
```
We can use pattern matching right up here to deconstruct that tuple struct directly in the argument list.
You can use this for any function.
This is not an `axum` feature.
It just happens to be really useful in the `axum` case.

```rust
(StatusCode, Json<User>)
```
And then you see the `create_user()` returns a tuple of status code, and we see this JSON object again with `User` as the parameterized type to `Json` here.
And as you would expect, this means it returns a status code, which is the `StatusCode` here comes from the `http` module of `axum`.
```rust
use axum::http::StatusCode;
```
And it defines all of the standard HTTP status codes.
I think it in fact is a re-export.
So the `http` module is just a re-export of the `http` crate, which has definitions for all of the different verbs and methods and things in the HTTP standard.
```rust
pub use http;
```
And you say the body of the response should be the JSON encoding of a `User` struct.

So the way the method router actually works is it internally contains a sort of map based on the method of the incoming request.
So it's basically a middleware.
When it receives a request, it'll look at its internal map from method to handler, and it will look at the method of the request and it will just call the appropriate handler's handling of that request.
So the request comes in here, and then it just gets routed to the right handler.
But the router is really just a thing that holds many handler functions and chooses which one to call depending on the properties of the request.

### Starting the actual server

`axum::Server` is really a `hyper` server as you can see here.
So this is just reusing the `hyper` service stack.
And `bind` just returns you back a hyper server.
Specifically a hyper server builder.
And then when you actually want to start serving requests, you call `serve` on that builder.
So if I go to the `hyper` docs, then go to `server`, and go to `Server`, you see the `bind` returns a `Builder`.
And `Builder` has a `serve` method with lots and lots of trait bounds.
But essentially, it takes a thing that can make a service.
And what that means is every time a new connection comes in, it calls this thing to produce a service, a service being an implementer of the service trait.
And so each connection gets its own instance of a service that gets called for every request on that connection.
And so `serve` basically never returns.
`serve` is just like start the loop to listen for connections now.
And all of these bounds are all about like making sure that you actually have an HTTP connection that you can read and write from, that the errors are sendable across thread boundaries and all that.

## `tower::Service`

Like people don't implement the `tower::Service` trait because of `axum`, they implement it for all sorts of other reasons.
But if you're using `axum`, you can make use of those implementations, which allows you to do things like share middleware with applications written using `hyper` or `tonic`.

So let's now look at the `Service` trait from the `tower` crate.
So the `tower` crate mostly exists around this trait.
Like it exists to define this trait and to allow an ecosystem to develop around this `Service` trait.
And the `Service` trait fundamentally is a trait that takes in requests and asynchronously produces responses.
And these aren't tied to HTTP.
They can be requests of any kind, responses of any kind.
And the idea is that when you `call`, what you get back is a `Future`, where that `Future` type's output is a `Result` that is either a `Response` or an `Error`.
So it's almost the most generic way you can think of expressing a service, which is the intent.
That you have this trait that everyone can implement if they have anything that deals with requests and responses.
[Inventing the Service trait](https://tokio.rs/blog/2021-05-14-inventing-the-service-trait) goes through how we arrived at the exact structure of the `Service` trait. 

The basic way to think about service is that `tower::Service` is roughly equivalent to something that implements
```rust
async fn f(Request) -> Result<Response, E> for some <Request, E>
```
What's really nice about this is that they compose.
So imagine that you have one function or one type that implements the `tower::Service` trait.
They compose through the use of a separate trait, which we haven't talked about yet, called `Layer`.
So this is a separate `tower::Layer` trait that let's you take services and sort of merge them.

I more want to give you the mental model for what these services are.
They are ways to express asynchronous mappings from requests to responses.
And every `axum` handler is fundamentally one of these, or rather, to be more accurate, gets turned into one of these.

```rust
async fn root() -> &'static str {
    "Hello, World!"
}
```
Like we have this function, but how does that turn into like something that takes an HTTP request, and turns it into an HTTP response.
```rust
async fn root(http::request::Request) -> http::response::Response
```
Like how does this transformation happen?
It is somehow transformed by `axum`.
Somehow that happens.
And somehow that also happens for this.
```rust
async fn create_user(
    Json<payload>: Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    ...
}
```
Which is constructed very differently, somehow also gets turned into that same `tower::Service`, that same kind of asynchronous mapping.

And if we go back to somthing like a router.
So we were talking earlier about what does `get` really do?
It gets a request in.
And it's eventually going to return a response.
It has like a map somewhere of all of the inner handlers.
In fact, maybe even let's reconstruct `MethodHandler`.

## The `axum::handler::Handler` trait and its impls

How does that mapping happen from handler to a `tower::Service`?
The thing that you return has to be possible to turn into a response.
So there's like some path here where you can start to sort of see the shape.
So a handler is something that, it has a `Future`.
You can call it with a request and state, we don't know what state is yet, and returns one of these futures.
And the output from that future has to be an `axum::response::Response`.
So it takes an `axum` request or an HTTP request and returns an `axum` response.
This looks a lot like `Service`.
In fact, we see here `HandlerService`.
`HandlerService` implements `tower::Service`.
So there's like some path here where you can start to sort of see the shape.
That if we go back here to the method signature of the `axum::routing::method_routing::get`.
So `get` takes a `handler: H`, where `H` implements `Handler`.
So what that means is this `async fn root() -> &'static str` implements `Handler`.
Otherwise, we wouldn't be able to call `get` with `root`.
Otherwise we wouldn't be able to call this function because that is the trait bound of `get`.
And similarly, `post` is passing `create_user`.
Then that means the `create_user` function also implements `Handler` somehow.

So this `Handler` trait is somehow implemented for all of our random functions.
How does that work?
How is that almost any function we write here, this is not quite true, but it feels anything we wanna put here just kind of works.
It somehow is a valid handler.
Well, now we get to the key.
So the `Handler` trait is automatically implemented for a lot of types.
And if you look carefully, you see there's a pattern here.
Like it's not random types, it's for a bunch of tuple types with an increasing number of tuples.

So if we scroll down here, let's look for something that is a meaningful length, like this one
```rust
impl<F, Fut, S, Res, M, T1, T2> Handler<(M, T1, T2), S> for F
where
    F: FnOnce(T1, T2) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Res> + Send,
    S: Send + Sync + 'static,
    Res: IntoResponse,
    T1: FromRequestParts<S> + Send,
    T2: FromRequest<S, M> + Send,
```
So `Handler` is implemented for any `F`, `F` is generic here, for any `F`, where `F` implements `FnOnce` that takes two arguments, `T1` and `T2`, notice that this matches with what this first type is in the `Handler` trait.
This `Handler` trait is implemented for any function `F` that takes two arguments, and returns a `Fut`.
`Fut` is a generic where `Fut` is a `Future`.
And the output of that `Future`, the thing that the `Future` returns is a `Res`, which is a generic where `Res` implements `IntoResponse`.
And `IntoResponse`, as you can guess from the name, is something that can be turned into an HTTP response.
And then you see the last bit where T1 and T2 implement `FromRequestParts` and `FromRequest`.
Like the `Handler` trait is automatically implemented for functions of basically any argument length, as long as all of the arguments implement `FromRequstParts` and the last one implements `FromRequest` and the response implements `IntoResponse`.

### What is `IntoResponse`?

`axum::response::IntoResponse` is just something that can be turned into a response where response here is HTTP response.

### What is extractor?

And so at this point, we actually know what extractor is.
An extractor is something that implements `FromRequestParts` or `FromRequest`.
And so it can take the parts of a request or the whole request and turn it into something that can appear in the argument list for a handler and have it still implement the overall `Handler` trait.

### The `impl_handler` macro

The last part that was really useful to me actually when I first found it is the `impl_handler` macro.
```rust
macro_rules! impl_handler {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, Fut, S, Res, M, $($ty,)* $last> Handler<(M, $($ty,)* $last,), S> for F
        where
            F: FnOnce($($ty,)* $last,) -> Fut + Clone + Send + 'static,
            Fut: Future<Output = Res> + Send,
            S: Send + Sync + 'static,
            Res: IntoResponse,
            $( $ty: FromRequestParts<S> + Send, )*
            $last: FromRequest<S, M> + Send,
        {
            type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

            fn call(self, req: Request, state: S) -> Self::Future {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*

                    let req = Request::from_parts(parts, body);

                    let $last = match $last::from_request(req, state).await {
                        Ok(value) => value,
                        Err(rejection) => return rejection.into_response(),
                    };

                    let res = self($($ty,)* $last,).await;

                    res.into_response()
                })
            }
        }
    };
}
```
So this macro right here is what I would argue is the heart of `axum`.
The `impl_handler` is the thing that actually implements the `Handler` trait for all of these different function types.
```rust
$( $ty: FromRequestParts<S> + Send, )*
$last: FromRequest<S, M> + Send,
```
All of the types have to implement `FromRequestParts`, except for the last one that has to implement `FromRequest`.
So this is really just the thing that generates the `impl` blocks we looked at earlier.

But then let's look the actual `call` here.
So what does it do?
Well, this is the sort of equivalent of the `tower::Service` trait.
When we get a request in, it splits that request into parts and `into_parts` is really it splits the body of the request from everything else about the request. 
So things like the method, the URI, any additional headers, all of that goes into parts and then the body is kept separately.
And the reason it does this, and the whole reason why `FromRequestParts` is separate from `FromRequest` is because you can only read the body once.
So that's why they sort of enforce this `FromRequestParts` only gets access to the metadata of the request, the `parts`.
And the `$last` thing, the thing that gets to implement the `FromRequest` trait, that one is also given access to the body.
When `axum` runs the handler, it's going to run all of these things, all the argument extractors first, and they don't have access to the body, and then it'll run the last one, which does have access to the body.

### `debug_handler`

And `debug_handler` is an attribute that you can add to your handler, and it will generally give you better errors about what went wrong.

### `IntoResponse`

And you'll see a similar thing if you look at `IntoResponse`.

## The `State` extractor

Extractors are just things that implement `FromRequestParts`.
That's the only requirement for something to be an extractor, is that it can extract things from either a full request, including the body, or from the parts of a request.
There's an extractor called `State` which works a little bit differently.
So when you want to have shared state between your handlers, think something like a pool of database connections or even just a data structure behind a mutex or whatever it might be.
You construct the shared state.
You start the router, you call `route`, and you call `.with_state`.
So this basically hands that state into the router so that it is going to pass it to all the handlers.
```rust
struct AppState {
    // ...
}

let shared_state = Arc::new(AppState { /* ... */ });

let app = Router::new()
    .route("/", get(handler))
    .with_state(shared_state);
```
Once you call `.with_state` like this, you're able to now in your handlers declare in your argument list, I want `State` and that is one of these.
```rust
async fn handler(
    State(state): State<Arc<AppState>>,
) {
    // ...
}
```
And in practice, what that means is handlers are going to get separate clones of the state.
And that's why the `Arc` is here.
Because if you have multiple handlers, like those handlers might be called in parallel.
You might have multiple handlers that all want access to the state.
And so it needs to be cloned in order to be shared.
But this `Arc` is an easy way to get something that's trivially cloneable in order to share them.
So the requirement is that for anything you pass in with state like this, it has to implement `Clone`.

But how does this work?
Like so far when we say `FromRequestParts` or `FromRequest`, it's all been about things that are in the request.
How are they getting access to this state thing?
The `State` extractor also implements `FromRequestParts`.

How does the state make it from the router and into the handlers when they actually get called?
There's this innocent looking function `with_state`.
And what `with_state` does is it takes a handler, anything that implements the `Handler` trait, and it gives you a `HandlerService`.
So you provide it with the state and it gives you a service.
So a `HandlerService` holds the handler and the state.
```rust
pub struct HandlerService<H, T, S> {
    handler: H,
    state: S,
    _marker: PhantomData<fn() -> T>,
}
```
And then it implements `tower::Service` for `HandlerService`.

But this piece is important to understand, which is that once handlers are provided with their state, they stop being handlers and start being self-standing services that do not need their state anymore because they already have it internally cloned.
