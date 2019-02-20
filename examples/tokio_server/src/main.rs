extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
    let addr = "127.0.0.1:1234".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    // Convert the `TcpListener` to a stream of incoming connections
    let server = listener
        .incoming()
        // for each connection
        .for_each(|socket| {
            // split the socket stream into readable and writable parts
            let (reader, writer) = socket.split();
            // copy bytes from the reader into the writer
            let amount = io::copy(reader, writer);

            let msg = amount.then(|result| {
                match result {
                    Ok((_amount, _, _)) => {} //println!("wrote {} bytes", _amount),
                    Err(e) => println!("error: {}", e),
                }

                Ok(())
            });

            // spawn the task that handles the client connection socket on to the
            // tokio runtime.
            tokio::spawn(msg);
            Ok(())
        })
        .map_err(|err| {
            // Handle error by printing
            println!("accept error = {:?}", err);
        });

    println!("server running on {}", addr);

    // This does a few things:
    // * Start the Tokio runtime
    // * Spawns the `server` task onto the runtime.
    // * Blocks the current thread until the runtime becomes idle, i.e. all
    //   spawned tasks have completed.
    tokio::run(server);
}
