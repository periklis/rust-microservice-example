#![deny(warnings)]
extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use chrono::{DateTime, Utc};
use futures::{Async, Future, IntoFuture, Poll};
use futures::future::{FutureResult, ok};
use hyper::{Body, Method, Request, Response, Server};
use hyper::Error as hyperError;
use hyper::service::Service;
use std::fmt;


/// Serde is a type-safe serializing/deserializing library
///
/// Example:
/// ```
/// let json = json!(Entity{ name: "test", count: 32 })
/// let entity: Entity = serde_json::from_str(&json).unwrap();
/// ```
mod serde_impls {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

/// Definitions for entities can be done with:
/// - structs for product types
/// - enums for sum types
#[derive(Serialize, Deserialize)]
pub struct Region {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[serde(with = "serde_impls")]
    pub created: DateTime<Utc>
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.created)
    }
}


///
/// Asynchronous computation e.g. repositories are implemented as Futures.
///
#[derive(Debug, Clone)]
pub struct RegionRepository<'a> {
    pub host: &'a str,
    pub port: i32
}

impl<'a> Future for RegionRepository<'a> {
    type Item = String;
    type Error = hyperError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let entity = json!(Region {
            id: 1234,
            name: "Crete".to_string(),
            description: "Biggest greek Island".to_string(),
            created: Utc::now()
        });

        Ok(Async::Ready(entity.to_string()))
    }
}


///
/// Connecting every request with a route can be done in a service which in turn
/// are Futures.
///
#[derive(Debug)]
struct App<'a> {
    repository: RegionRepository<'a>
}

impl<'a> Service for App<'a> {
    type ReqBody = Body;
    type ResBody = Body;
    type Error   = hyperError;
    type Future  = Box<Future<Item = Response<Self::ResBody>, Error = Self::Error> + 'a + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => {
                let repository = self.repository.clone();
                Box::new(repository.map(|entity| Response::new(Body::from(entity))))
            }
            _ => {
                let res = Response::builder()
                    .status(404)
                    .body(Body::from("Not found"))
                    .unwrap();

                Box::new(ok(res))
            }
        }
    }
}

impl<'a> IntoFuture for App<'a> {
    type Future = FutureResult<Self::Item, Self::Error>;
    type Item = Self;
    type Error = hyperError;

    fn into_future(self) -> Self::Future {
        ok(self)
    }
}

///
/// Select a runtime and connecting to an address.
///
fn main() {
    pretty_env_logger::init();

    let addr = ([127, 0, 0, 1], 1337).into();

    let server = Server::bind(&addr)
        .serve(|| {
            let repository = RegionRepository { host: "localhost", port: 9200 };
            App { repository }
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    hyper::rt::run(server);
}
