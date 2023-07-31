# GraphQL in Action

# 1 Introduction to GraphQL

## 1.1 What is GraphQL?

The word *graph* in GraphQL comes from the fact that the best way to represent data in the real world is with a graph-like data structure.
If you analyze any data model, big or small, you'll always find it to be a graph of objects with many relations between them.

Note that the *graph* in GraphQL does not mean that GraphQL can only be used with a "graph database."

The *QL* in GraphQL might be a bit confusing, though.
Yes, GraphQL is a query language for data APIs, but that's only from the perspective of the frontend consumer of those data APIs.
GraphQL is also a runtime layer that needs to be implemented on the backend, and that layer is what makes the frontend consumer able to use the new language.

The GraphQL language is designed to be declarative, flexible, and efficient.
Developers of data API consumers (like mobile and web applications) can use that language to request the data they need in a language close to how they think about data in their heads instead of a language related to how the data is stored or how data relations are implemented.

On the backend, a GraphQL-based stack needs a runtime. That runtime provides a structure for servers to describe the data to be exposed in their APIs.
This structure is what we call a *schema* in the GraphQL world.
An API consumer can then use the GraphQL language to construct a text request representing their exact data needs.
The client sends that text request to the API service through a transport channel (for example, HTTPS).
The GraphQL runtime layer accepts the text request, communicates with other services in the backend stack to put together a suitable data response, and then sends that data back to the consumer in a format like JSON.

### 1.1.1 The big picture

For GraphQL, we are specifically talking about the API type used to read and modify data, which is usually referred to as a *data API*.

GraphQL is one option out of many that can be used to provide applications with programmable interfaces to read and modify the data the applications need from data services.
Other options include REST, SOAP, XML, and even SQL itself.  

GraphQL for client applications is another language they can use to express their data requirements.  

A GraphQL server can understand this syntax and translate it into something the data storage engine can understand (for example, the GraphQL server might translate the query into SQL statements for a relational database).
Then, the GraphQL server can take what the storage engine responds with, translate it into something like JSON or XML, and send it back to the client application.  

### 1.1.2 GraphQL is a specification

GraphQL is an evolving language, but the specification document was a genius start for the project because it defined standard rules and practices that all implementers of GraphQL runtimes must adhere to.
There have been many implementations of GraphQL libraries in many different programming languages, and all of them closely follow the specification document and update their implementations when that document is updated.

### 1.1.3 GraphQL is a language

### 1.1.4 GraphQL is a service

If we teach a client application to speak the GraphQL language, it will be able to communicate any data requirements to a backend data service that also speaks GraphQL.
To teach a data service to speak GraphQL, you implement a runtime layer and expose that layer to the clients that want to communicate with the service.
Think of this layer on the server side as simply a translator of the GraphQL language, or a GraphQL-speaking agent that represents the data service.
GraphQL is not a storage engine, so it cannot be a solution on its own.
This is why you cannot have a server that speaks just GraphQL; you need to implement a translating runtime layer.

A GraphQL service can be written in any programming language, and it can be conceptually split into two major parts, structure and behavior:
* The *structure* is defined with a strongly typed *schema*.
A GraphQL schema is like a catalog of all the operations a GraphQL API can handle.
It simply represents the capabilities of an API.
GraphQL client applications use the schema to know what questions they can ask the service.
The typed nature of the schema is a core concept in GraphQL.
The schema is basically a graph of fields that have types; this graph represents all the possible data objects that can be read (or updated) through the service.
* The *behavior* is naturally implemented with functions that in the GraphQL world are called *resolver functions*.
They represent most of the smart logic behind GraphQL's power and flexibility.
Each field in a GraphQL schema is backed by a resolver function.
A resolver function defines what data to fetch for its field.

A resolver function represents the instructions on how and where to access raw data.
For example, a resolver function might issue a SQL statement to a relational database, read a file's data directly from the operating system, or update some cached data in a document database.
A resolver function is directly related to a field in a GraphQL request, and it can represent a single primitive value, an object, or a list of values or objects.

## 1.2 Why GraphQL?

Experienced backend developers have been creating efficient technologies for data APIs since long before GraphQL.
So why do we need a new technology?
If you asked me to answer the "Why GraphQL?" question with a single word, that word would be *standards*.

GraphQL provides comprehensive standards and structures to implement API features in maintainable and scalable ways.
GraphQL makes it mandatory for data API servers to publish documentation (the schema) about their capabilities.
That schema enables client applications to know everything available for them on these servers.
The GraphQL standard schema has to be part of every GraphQL API. Clients can ask the service about its schema using the GraphQL language.

Other solutions can be made better by adding similar documentation.
The unique thing about GraphQL here is that the documentation is part of how you create the API service.
You cannot have out-of-date documentation.
You cannot forget to document a use case.
You cannot offer different ways to use APIs, because you have standards to work with.
Most important, you do not need to maintain the documentation of your API separately from that API.
GraphQL documentation is built-in, and it's first class.

The mandatory GraphQL schema represents the possibilities and the limits of what can be answered by the GraphQL service.
But there is some flexibility in how to use the schema because we are talking about a graph of nodes, and graphs can be traversed using many paths.
This flexibility is one of the great benefits of GraphQL because it allows backend and frontend developers to make progress in their projects without needing to constantly coordinate their progress with each other.
It basically decouples clients from servers and allows both of them to evolve and scale independently.
This enables faster iteration in both frontend and backend products.

# 2 Exploring GraphQL APIs

The easiest way to start learning about the powerful features of the GraphQL language is to use its feature-rich interactive in-browser IDE.
This IDE uses GraphQL's type system to provide features you can use to explore what you can do with GraphQL and to write and test your GraphQL requests without leaving your browser.
Using this IDE, we will continue to explore examples of GraphQL queries and mutations.

## 2.1 The GraphiQL editor

When thinking about the requests your client applications need to make to servers, you can benefit from a graphical tool to first help you come up with these requests and then test them before committing to them in application code.
Such a tool can also help you improve these requests, validate your improvements, and debug any requests that are running into problems.
In the GraphQL world, this tool is called GraphiQL (with an i before the QL and pronounced "graphical").
GraphiQL is an open source web application (written with React.js and GraphQL) that can be run in a browser.

## 2.2 The basics of the GraphQL language

### 2.2.1 Requests  

At the core of a GraphQL communication is a *request* object.
The source text of a GraphQL request is often referred to as a *document*.
A document contains text that represents a request through operations like queries, mutations, and subscriptions.
In addition to the main operations, a GraphQL document text can contain fragments that can be used to compose other operations, as we will see in the next chapter.

### 2.2.2 Fields

## 2.3 Examples from the GitHub API

### 2.3.1 Reading data from GitHub

### 2.3.3 Introspective queries

GraphQL APIs support introspective queries that can be used to answer questions about the API schema.
This introspection support gives GraphQL tools powerful functionality, and it drives the features we have been using in the GraphiQL editor.

# 3 Customizing and organizing GraphQL operations

## 3.1 Customizing fields with arguments

The fields in a GraphQL operation are similar to functions.
They map input to output. A function input is received as a list of argument values.
Just like functions, we can pass any GraphQL field a list of argument values.
A GraphQL schema on the backend can access these values and use them to customize the response it returns for that field.  

### 3.1.1 Identifying a single record to return

Every API request that asks for a single record from a collection needs to specify an identifier for that record.
This identifier is usually associated with a unique identifier for that record in the server's database, but it can also be anything else that can uniquely identify the record.

### 3.1.2 Limiting the number of records returned by a list field

It is usually a bad idea to leave a general API capability for listing records in a collection without a limit.
You do not want a client to be able to fetch more than a few hundred records at a time, because that would put your API server at risk of resource exhaustion and does not scale well.

### 3.1.3 Ordering records returned by a list field

### 3.1.4 Paginating through a list of records

### 3.1.5

### 3.1.6

## 3.2 Renaming fields with aliases

## 3.3 Customizing responses with directives
