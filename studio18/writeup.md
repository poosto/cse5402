### Studio 18

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Try to compile and run your program, and as the answer to this exercise please show the error that occurred.*
```shell
❯ cargo check
    Checking rustasyncserver v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio18/rustasyncserver)
error[E0433]: failed to resolve: use of undeclared crate or module `async_std`
 --> src/main.rs:1:5
  |
1 | use async_std::net::TcpListener;
  |     ^^^^^^^^^ use of undeclared crate or module `async_std`

For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustasyncserver` (bin "rustasyncserver") due to previous error
```

3) *As the answer to this exercise please show the output that the program produced when it ran.* \
Note that I had to use the line `async-std = { version = "=1.7.0", features = ["unstable"] }
` to pin the version to `1.7.0` exactly to fix some build issues.
```shell
warning: unused import: `async_std::net::TcpListener`
 --> src/main.rs:1:5
  |
1 | use async_std::net::TcpListener;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `rustasyncserver` (bin "rustasyncserver") generated 1 warning (run `cargo fix --bin "rustasyncserver"` to apply 1 suggestion)
    Finished dev [unoptimized + debuginfo] target(s) in 4.12s
     Running `target/debug/rustasyncserver`
Hello, world!
```

4) *Try to compile and run your program, and as the answer to this exercise please show the error that occurred.*
```shell
❯ cargo run
   Compiling rustasyncserver v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio18/rustasyncserver)
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> src/main.rs:5:17
  |
3 | fn main() {
  |    ---- this is not `async`
4 |     let bind_future = TcpListener::bind("127.0.0.1:7777");
5 |     bind_future.await;
  |                 ^^^^^ only allowed inside `async` functions and blocks

For more information about this error, try `rustc --explain E0728`.
error: could not compile `rustasyncserver` (bin "rustasyncserver") due to previous error
```

5) *Compile and run your program, and as the answer to this exercise please show the output the program produced.*
```rust
use async_std::net::TcpListener;

fn main() {
    let bind_future = TcpListener::bind("127.0.0.1:7777");

    match async_std::task::block_on(bind_future) {
        Err(e) => eprintln!("Failed to bind: {e}"),
        Ok(_) => println!("Server bound successfully"),
    };
}
```
```shell
❯ cargo run
   Compiling rustasyncserver v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio18/rustasyncserver)
    Finished dev [unoptimized + debuginfo] target(s) in 0.84s
     Running `target/debug/rustasyncserver`
Server bound successfully
```

6) *Compile and run your program, and as the answer to this question please show the output it produced.*
```rust
use async_std::net::TcpListener;

async fn run_server() -> std::io::Result<String> {
    TcpListener::bind("127.0.0.1:7777").await?;

    Ok(String::from("Success!"))
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
   Compiling rustasyncserver v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio18/rustasyncserver)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/rustasyncserver`
Success!
```

7) *As the answer to this exercise please show the code you wrote for the run_server function.*
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

8) *As the answer to this exercise please show the output that both the client and server programs produced.*
```rust
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
```
```shell
CSE 5402/studio18/rustasyncclient on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/rustasyncclient`
Connected to server
Sent: Hello from async client :)

CSE 5402/studio18/rustasyncclient on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/rustasyncclient`
Connected to server
Sent: Hello from async client :)

CSE 5402/studio18/rustasyncclient on  main [?] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient`
Connected to server
Sent: Hello from async client :)
```
```shell
❯ cargo run
   Compiling rustasyncserver v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio18/rustasyncserver)
    Finished dev [unoptimized + debuginfo] target(s) in 0.67s
     Running `target/debug/rustasyncserver`
Bound successfully. Waiting for connections...
Received: Hello from async client :)
Received: Hello from async client :)
Received: Hello from async client :)
```
