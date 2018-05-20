#![feature(rust_2018_preview, rust_2018_idioms, proc_macro, generators)]

extern crate chrono;
extern crate futures;

use chrono::offset::Utc;
use chrono::DateTime;
use futures::{Async, Future, Poll, task};
use futures::future::select_all;
use futures::executor::ThreadPoolBuilder;
use std::io::Error as ioError;

#[derive(Debug)]
struct AskTimeserver<'a> {
    host: &'a str,
    time: DateTime<Utc>
}

impl AskTimeserver<'a> {
    fn new(host: &str) -> AskTimeserver {
        AskTimeserver { host: host, time: Utc::now() }
    }
}

impl Future for AskTimeserver<'a> {
    type Item = DateTime<Utc>;
    type Error = ioError;

    fn poll(&mut self, _cx: &mut task::Context) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready(Utc::now()))
    }
}

#[derive(Debug)]
struct Timer {
    now: DateTime<Utc>,
}

impl Timer {
    fn new() -> Timer {
        Timer { now: Utc::now() }
    }
}

impl Future for Timer {
    type Item = (String, DateTime<Utc>);
    type Error = ioError;

    fn poll(&mut self, cx: &mut task::Context) -> Poll<Self::Item, Self::Error> {
        let timeservers = vec!(
            AskTimeserver::new("time.apple.com"),
            AskTimeserver::new("time.ntp.org"),
            AskTimeserver::new("time-a-b.nist.gov"),
            AskTimeserver::new("time-b-b.nist.gov")
        );

        select_all(timeservers).poll(cx).map(|res| {
            res.map(|v| {
                (v.2.get(v.1).unwrap().host.to_string(), v.0)
            })
        }).map_err(|res| {
            res.0
        })
    }
}

fn main() {
    let mut pool = ThreadPoolBuilder::new()
        .pool_size(4)
        .name_prefix("rs-microservice")
        .create()
        .unwrap();

    let result = pool.run(Timer::new());

    println!("Result {:?}", result);
}
