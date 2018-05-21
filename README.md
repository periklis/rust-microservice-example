# rust-microservice-example
Second evaluation of Rust web ecosystem - Simple Microservice with Hyper, Serde, Futures

## Motivation

Since my last evaluation of the Rust web ecosystem in Summer 2017 and post Futures 0.2 release, this repository is another evaluation sample of the current state of the art. 
Basic requirements to write simple microservices:
- Low friction setup for a http server, with support for HTTP 1.1 & 2.0
- Easy setup routing with optional support for middleware
- Easy setup for execution contexts, e.g. easy switch between thread pools and single threaded execution
- Implementation with std-lang features of a domain's bounded context: Services, Repositories, Entities, Value Objects
- Straight forward serialization/deserialization for compound types w/o own impls for a wide range of std types.
- Fully support asynchronous I/O from Service down to repositories, client calls, heavy computations, aggregations, etc.

## Evaluation results

- Setting up a http server with hyper is still a frictionful job, because of:
  - Current 0.11 release is lacking HTTP 2 and bases on Futures 0.1 which in turn is missing executors
  - Master is upcoming 0.12 which has support for HTTP 2, but still locked in Futures 0.1 because tokio is not upgraded to Futures 0.2, yet.
  - Support for middlewares is still a manual job
  - Documentation and examples are work in progress for 0.12.
  - Implementation of a `impl Service` is not as easy as passing a function to `fn service_fn`
- Routing support is easy
- Switching between execution context is easy thanks to `hyper::server::Http::executor`
- Implementing simple services, repositories and wiring dependencies still a hurdle due to lifetimes, but managable.
- Serde is still rock solid and good companion.
- Full asynchronous I/O support is questionable, since other crates not rebased upon Futures 0.2, yet.
