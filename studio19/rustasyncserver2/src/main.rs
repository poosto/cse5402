use std::net::Shutdown;

use async_std::{io, net::TcpListener, prelude::*, task};

async fn run_server() -> std::io::Result<String> {
    let listener = TcpListener::bind("127.0.0.1:7777").await?;

    println!("Bound successfully. Waiting for connections...");

    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        // Don't ? the stream object because we don't want to shut the server down
        // from a single failed connection.
        match stream {
            Ok(mut stream) => {
                let mut data = String::new();

                match stream.read_to_string(&mut data).await {
                    Ok(_) => {
                        data = data.trim().to_string();

                        println!("Received: {data}");

                        if data == "quit" {
                            break;
                        } else {
                            task::spawn(async move {
                                stream.write_all(format!("{data}\n").as_bytes()).await?;
                                println!("Echoed back: {data}");

                                stream.shutdown(Shutdown::Both)?;

                                Ok::<(), io::Error>(())
                            });
                        }
                    }
                    Err(e) => eprintln!("Failed to read from socket: {e}"),
                }
            }
            Err(e) => eprintln!("Failed to accept connection: {e}"),
        }
    }

    Ok(String::from("Server shut down"))
}

fn main() {
    let bind_future = run_server();

    match async_std::task::block_on(bind_future) {
        Err(e) => eprintln!("Server error: {e}"),
        Ok(msg) => println!("{msg}"),
    };
}
