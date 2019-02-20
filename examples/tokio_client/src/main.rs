extern crate futures;
extern crate tokio;

use futures::future::Future;
use futures::stream::{self, Stream};
use time::PreciseTime;
use tokio::io;
use tokio::net::TcpStream;
use tokio_retry::strategy::{jitter, ExponentialBackoff};
use tokio_retry::Retry;

fn action() -> impl Future<Item = (), Error = ()> {
    let addr = "127.0.0.1:1234".parse().unwrap();
    TcpStream::connect(&addr)
        .and_then(|stream| {
            // println!("connected");

            io::write_all(stream, "hello world\n").then(|_result| {
                // println!("wrote to connection; success={:?}", _result.is_ok());
                Ok(())
            })
        })
        .map_err(|err| {
            println!("connection error = {:?}", err);
        })
}

fn main() {
    let number_of_connections = 100_000;
    let retry_strategy = ExponentialBackoff::from_millis(10).map(jitter).take(3);

    let client = stream::iter_ok(0..number_of_connections)
        .for_each(move |_| Retry::spawn(retry_strategy.clone(), action).then(|_| Ok(())));

    let start = PreciseTime::now();
    tokio::run(client);
    let end = PreciseTime::now();

    println!(
        "{} seconds for {} connections. {} connections per seccond)",
        start.to(end),
        number_of_connections,
        number_of_connections as f64 / start.to(end).num_seconds() as f64,
    );
}
