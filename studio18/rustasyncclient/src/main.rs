use async_std::{net::TcpStream, prelude::*, task::block_on};

fn main() {
    let mut stream = match block_on(TcpStream::connect("127.0.0.1:7777")) {
        Ok(stream) => {
            println!("Connected to server");
            stream
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {e}");
            return;
        }
    };

    let message = "Hello from async client :)";

    match block_on(stream.write_all(message.as_bytes())) {
        Ok(()) => println!("Sent: {message}"),
        Err(e) => eprintln!("Failed to send message: {e}"),
    }
}
