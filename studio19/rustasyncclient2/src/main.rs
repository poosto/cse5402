use std::env::args;

use async_std::{
    io::BufReader,
    net::{Shutdown, TcpStream},
    prelude::*,
    task::block_on,
};

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

    let message = if args().count() == 1 {
        "quit".to_string()
    } else {
        args().skip(1).collect::<Vec<_>>().join(" ")
    };

    match block_on(stream.write_all(message.as_bytes())) {
        Ok(()) => println!("Sent: {message}"),
        Err(e) => eprintln!("Failed to send message: {e}"),
    }

    match stream.shutdown(Shutdown::Write) {
        Ok(()) => println!("Successfully shutdown write stream"),
        Err(e) => eprintln!("Failed to shutdown client: {e}"),
    }

    let mut reader = BufReader::new(&stream);
    let mut line = String::new();

    match block_on(reader.read_line(&mut line)) {
        Ok(0) => println!("Received EOF"),
        Ok(size) => println!("Received {size} bytes: {}", line.trim()),
        Err(e) => eprintln!("Failed to receive message: {e}"),
    }
}
