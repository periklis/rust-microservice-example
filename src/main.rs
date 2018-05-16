#![feature(proc_macro, generators)]

extern crate chrono;
extern crate futures_await as futures;
extern crate tokio_core;

use chrono::DateTime;
use chrono::offset::Utc;
use futures::{Async, Future, Poll};
// use futures::done;
// use futures::prelude::*;
// use futures::future::{err, ok};
use tokio_core::reactor::Core;
// use std::error::Error;


#[derive(Debug)]
struct Timer {
    now: DateTime<Utc>
}

impl Timer {
    fn new() -> Timer {
        Timer { now: Utc::now() }
    }
}

impl Future for Timer {
    type Item = DateTime<Utc>;
    type Error = String;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready(Utc::now()))
    }
}


fn main() {
    let mut reactor = Core::new().unwrap();

    let retval = reactor.run(Timer::new());
    println!("Core returned {:?}!", retval);
}
