### Studio 19

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *As the answer to this exercise please show the errors that were produced.*
Both programs fail to compile because the new packages do not list `async_std` as a dependency in their `Cargo.toml` files.

Server:
```shell
❯ cargo run
   Compiling rustasyncserver2 v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio19/rustasyncserver2)
error[E0433]: failed to resolve: use of undeclared crate or module `async_std`
 --> src/main.rs:1:5
  |
1 | use async_std::{net::TcpListener, prelude::*};
  |     ^^^^^^^^^ use of undeclared crate or module `async_std`

error[E0432]: unresolved import `async_std::net::TcpListener`
 --> src/main.rs:1:17
  |
1 | use async_std::{net::TcpListener, prelude::*};
  |                 ^^^^^^^^^^^^^^^^

error[E0433]: failed to resolve: use of undeclared crate or module `async_std`
  --> src/main.rs:32:11
   |
32 |     match async_std::task::block_on(bind_future) {
   |           ^^^^^^^^^ use of undeclared crate or module `async_std`

Some errors have detailed explanations: E0432, E0433.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `rustasyncserver2` (bin "rustasyncserver2") due to 3 previous errors
```

Client:
```shell
❯ cargo run
   Compiling rustasyncclient2 v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio19/rustasyncclient2)
error[E0433]: failed to resolve: use of undeclared crate or module `async_std`
 --> src/main.rs:1:5
  |
1 | use async_std::{net::TcpStream, prelude::*, task::block_on};
  |     ^^^^^^^^^ use of undeclared crate or module `async_std`

error[E0432]: unresolved imports `async_std::net::TcpStream`, `async_std::task::block_on`
 --> src/main.rs:1:17
  |
1 | use async_std::{net::TcpStream, prelude::*, task::block_on};
  |                 ^^^^^^^^^^^^^^              ^^^^^^^^^^^^^^

Some errors have detailed explanations: E0432, E0433.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `rustasyncclient2` (bin "rustasyncclient2") due to 2 previous errors
```

3) *As the answer to this exercise please show the output that was produced.*
I added this to the `Cargo.toml` of both projects to fix the error:
```toml
[dependencies]
async-std = { version = "=1.7.0", features = ["unstable"] }
```

Client (run 3 times):
```shell
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/rustasyncclient2`
Connected to server
Sent: Hello from async client :)

❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient2`
Connected to server
Sent: Hello from async client :)

❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient2`
Connected to server
Sent: Hello from async client :)
```

Server:
```shell
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/rustasyncserver2`
Bound successfully. Waiting for connections...
Received: Hello from async client :)
Received: Hello from async client :)
Received: Hello from async client :)
```

4) *As the answer to this exercise please show your client and server code and the output that each of them produced.*
Client (run three times):
```rust
use std::env::args;

use async_std::{
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
        Ok(()) => println!("Successfully flushed client"),
        Err(e) => eprintln!("Failed to shutdown client: {e}"),
    }
}
```
```shell
❯ cargo run hello world
   Compiling rustasyncclient2 v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio19/rustasyncclient2)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/rustasyncclient2 hello world`
Connected to server
Sent: hello world
Successfully flushed client

CSE 5402/studio19/rustasyncclient2 on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run hi
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient2 hi`
Connected to server
Sent: hi
Successfully flushed client

CSE 5402/studio19/rustasyncclient2 on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient2`
Connected to server
Sent: quit
Successfully flushed client
```

Server:
```rust
use async_std::{net::TcpListener, prelude::*};

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
                    Ok(_) => println!("Received: {data}"),
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
        Err(e) => eprintln!("Failed to bind: {e}"),
        Ok(msg) => println!("{msg}"),
    };
}
```
```shell
❯ cargo run
   Compiling rustasyncserver2 v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio19/rustasyncserver2)
    Finished dev [unoptimized + debuginfo] target(s) in 0.71s
     Running `target/debug/rustasyncserver2`
Bound successfully. Waiting for connections...
Received: hello world
Received: hi
Received: quit
```

5) *As the answer to this exercise, please show the output that was produced by both programs.*
Client (run 3 times):
```shell
CSE 5402/studio19/rustasyncclient2 on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run hello world
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/rustasyncclient2 hello world`
Connected to server
Sent: hello world
Successfully flushed client

CSE 5402/studio19/rustasyncclient2 on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run hi there
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient2 hi there`
Connected to server
Sent: hi there
Successfully flushed client

CSE 5402/studio19/rustasyncclient2 on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient2`
Connected to server
Sent: quit
Successfully flushed client
```

Server:
```shell
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/rustasyncserver2`
Bound successfully. Waiting for connections...
Received: hello world
Received: hi there
Received: quit
Server shut down
```

6) *As the answer to this exercise, please show the output that was produced by both programs.*
Client (run 3 times):
```shell
❯ cargo run hello
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient2 hello`
Connected to server
Sent: hello
Successfully flushed client
Received 6 bytes: hello

CSE 5402/studio19/rustasyncclient2 on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run hi there how are you
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient2 hi there how are you`
Connected to server
Sent: hi there how are you
Successfully flushed client
Received 21 bytes: hi there how are you

CSE 5402/studio19/rustasyncclient2 on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient2`
Connected to server
Sent: quit
Successfully flushed client
```

Server:
```shell
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncserver2`
Bound successfully. Waiting for connections...
Received: hello
Echoed back: hello
Received: hi there how are you
Echoed back: hi there how are you
Received: quit
Server shut down
```

7) *Confirm that the same output as in the previous exercise was produced, and as the answer to this exercise, please
show your updated server code.*
Client (run 3 times):
```rust
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
```
```shell
❯ cargo run hello
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient2 hello`
Connected to server
Sent: hello
Successfully shutdown write stream
Received 6 bytes: hello

CSE 5402/studio19/rustasyncclient2 on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run hi there how are you
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient2 hi there how are you`
Connected to server
Sent: hi there how are you
Successfully shutdown write stream
Received 21 bytes: hi there how are you

CSE 5402/studio19/rustasyncclient2 on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient2`
Connected to server
Sent: quit
Successfully shutdown write stream
Received EOF
```

Server:
```rust
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
```
```shell
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncserver2`
Bound successfully. Waiting for connections...
Received: hello
Echoed back: hello
Received: hi there how are you
Echoed back: hi there how are you
Received: quit
Server shut down
```
