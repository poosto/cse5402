### Studio 20

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Compile and run your server program with and without a single command line argument, and as the answer to this exercise show the output that was produced in each case.*
```rust
use std::env;
use std::process::ExitCode;

const ERR_INVALID_ARGS: u8 = 1;
const ERR_CANNOT_PARSE_ARGS: u8 = 2;

const ARG_IDX_PORT_NUMBER: usize = 0;

fn run() -> Result<(), u8> {
    let program_name = env::args().next().unwrap_or_default();
    let program_args = env::args().skip(1).collect::<Vec<_>>();

    if program_args.len() != 1 {
        eprintln!("Usage: {program_name} port_number");
        return Err(ERR_INVALID_ARGS);
    }

    let port = program_args[ARG_IDX_PORT_NUMBER]
        .parse::<u16>()
        .map_err(|_| {
            eprintln!("Unable to parse port number");
            ERR_CANNOT_PARSE_ARGS
        })?;

    println!("{}", port);

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => ExitCode::from(e),
    }
}
```
```shell
CSE 5402/studio20/reactive_server on  main [⇡] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
   Compiling reactive_server v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio20/reactive_server)
    Finished dev [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/reactive_server`
Usage: target/debug/reactive_server port_number

CSE 5402/studio20/reactive_server on  main [!⇡] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run hello world
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/reactive_server hello world`
Usage: target/debug/reactive_server port_number

CSE 5402/studio20/reactive_server on  main [!⇡] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run 45
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/reactive_server 45`
45

CSE 5402/studio20/reactive_server on  main [!⇡] is 📦 v0.1.0 via 🦀 v1.71.0
❯
```

3) *As the answer to this exercise please (1) indicate whether or not you saw an error like that, and if you did, show (2)
the error that was produced, (3) the output that was generated when you downgraded Tokio to version 1.53.0, and (4) the
output showing successful compilation of your program after that.* \
I did see an error like that even though I am running locally (since I am on 1.71.0):
```shell
CSE 5402/studio20/reactive_server on  main [!⇡] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
  Downloaded tokio v1.54.1
  Downloaded 1 crate (983.4 KB) in 0.24s
error: package `tokio v1.54.1` cannot be built because it requires rustc 1.85 or newer, while the currently active rustc version is 1.71.0
Either upgrade to rustc 1.85 or newer, or use
cargo update -p tokio@1.54.1 --precise ver
where `ver` is the latest version of `tokio` supporting rustc 1.71.0
```
To fix this, I ran the downgrade command:
```shell
❯  cargo update -p tokio@1.54.1 --precise 1.53.0
    Updating crates.io index
 Downgrading tokio v1.54.1 -> v1.53.0
```
This fixed the error:
```shell
CSE 5402/studio20/reactive_server on  main [!⇡] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
  Downloaded tokio v1.53.0
  Downloaded 1 crate (918.9 KB) in 0.21s
   Compiling pin-project-lite v0.2.17
   Compiling tokio v1.53.0
   Compiling reactive_server v0.1.0 (/Users/prestonmeek/Desktop/CLASS REPOS/CSE 5402/studio20/reactive_server)
    Finished dev [unoptimized + debuginfo] target(s) in 1.00s
     Running `target/debug/reactive_server`
Usage: target/debug/reactive_server port_number
```

4) *Compile and run your server with a valid port number like 8009 and an invalid one like 1001, and as the answer to
this exercise please show the output from those runs.*
```rust
use std::env;
use std::process::ExitCode;
use tokio::net::TcpListener;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum AppError {
    InvalidArgs = 1,
    CannotParseArgs = 2,
    BindFailed = 3,
}

impl From<AppError> for ExitCode {
    fn from(e: AppError) -> Self {
        ExitCode::from(e as u8)
    }
}

async fn run() -> Result<(), AppError> {
    let program_name = env::args().next().unwrap_or_default();
    let program_args = env::args().skip(1).collect::<Vec<_>>();

    let [port_arg] = program_args.as_slice() else {
        eprintln!("Usage: {program_name} port_number");
        return Err(AppError::InvalidArgs);
    };

    let port = port_arg.parse::<u16>().map_err(|_| {
        eprintln!("Unable to parse port number");
        AppError::CannotParseArgs
    })?;

    let addr = format!("127.0.0.1:{port}");

    let _listener = TcpListener::bind(&addr).await.map_err(|_| {
        eprintln!("Unable to bind to {addr}");
        AppError::BindFailed
    })?;

    println!("Listening on {addr}");

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => ExitCode::from(e),
    }
}
```
```shell
❯ cargo run 1001
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/reactive_server 1001`
Unable to bind to 127.0.0.1:1001

CSE 5402/studio20/reactive_server on  main [!⇡] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run 8009
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/reactive_server 8009`
Listening on 127.0.0.1:8009
```

5) *Compile and run your code, and test it by typing in a few other lines, and then typing in "quit" to confirm the
program exits successfully. As the answer to this exercise, please show the output from that run.*
```rust
use std::env;
use std::io::{self, BufRead};
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
static QUIT_FLAG: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum AppError {
    InvalidArgs = 1,
    CannotParseArgs = 2,
    BindFailed = 3,
    IoFailed = 4,
    JoinFailed = 5,
}

impl From<AppError> for ExitCode {
    fn from(e: AppError) -> Self {
        ExitCode::from(e as u8)
    }
}

async fn run() -> Result<(), AppError> {
    let program_name = env::args().next().unwrap_or_default();
    let program_args = env::args().skip(1).collect::<Vec<_>>();

    let [port_arg] = program_args.as_slice() else {
        eprintln!("Usage: {program_name} port_number");
        return Err(AppError::InvalidArgs);
    };

    let port = port_arg.parse::<u16>().map_err(|_| {
        eprintln!("Unable to parse port number");
        AppError::CannotParseArgs
    })?;

    let addr = format!("127.0.0.1:{port}");

    let _listener = TcpListener::bind(&addr).await.map_err(|_| {
        eprintln!("Unable to bind to {addr}");
        AppError::BindFailed
    })?;

    println!("Listening on {addr}");

    let (tx, mut rx) = mpsc::channel::<String>(1024);

    let tx_handle = std::thread::spawn(move || -> Result<(), AppError> {
        for line in io::stdin().lock().lines() {
            let line = line.map_err(|_| AppError::IoFailed)?;
            let is_quit = line == "quit";

            tx.blocking_send(line).map_err(|_| AppError::IoFailed)?;

            if is_quit {
                QUIT_FLAG.store(true, Ordering::Relaxed);
                break;
            }
        }
        Ok(())
    });

    let rx_handle = tokio::spawn(async move {
        while let Some(line) = rx.recv().await {
            println!("{line}");
        }
    });

    while !QUIT_FLAG.load(Ordering::Relaxed) {}

    tx_handle.join().map_err(|_| AppError::JoinFailed)??;
    rx_handle.await.map_err(|_| AppError::JoinFailed)?;

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => ExitCode::from(e),
    }
}
```
```shell
❯ cargo run 8008
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/reactive_server 8008`
Listening on 127.0.0.1:8008
hi
hi
hello blake!
hello blake!
quit
quit

CSE 5402/studio20/reactive_server on  main [!⇡] is 📦 v0.1.0 via 🦀 v1.71.0
❯
```

6) *Compile and run your server, and in a separate terminal window use your studio 18 client to connect to the server
and confirm it is accepting connections. As the answer to this exercise, please show the output the server produced
during those tests.*
```rust
use std::env;
use std::io::{self, BufRead};
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

static QUIT_FLAG: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum AppError {
    InvalidArgs = 1,
    CannotParseArgs = 2,
    BindFailed = 3,
    IoFailed = 4,
    JoinFailed = 5,
    ClientConnectFailed = 6,
}

impl From<AppError> for ExitCode {
    fn from(e: AppError) -> Self {
        ExitCode::from(e as u8)
    }
}

async fn run() -> Result<(), AppError> {
    let program_name = env::args().next().unwrap_or_default();
    let program_args = env::args().skip(1).collect::<Vec<_>>();

    let [port_arg] = program_args.as_slice() else {
        eprintln!("Usage: {program_name} port_number");
        return Err(AppError::InvalidArgs);
    };

    let port = port_arg.parse::<u16>().map_err(|_| {
        eprintln!("Unable to parse port number");
        AppError::CannotParseArgs
    })?;

    let addr = format!("127.0.0.1:{port}");

    let listener = TcpListener::bind(&addr).await.map_err(|_| {
        eprintln!("Unable to bind to {addr}");
        AppError::BindFailed
    })?;

    println!("Listening on {addr}");

    let (tx, mut rx) = mpsc::channel::<String>(1024);

    let tx_handle = std::thread::spawn(move || -> Result<(), AppError> {
        for line in io::stdin().lock().lines() {
            let line = line.map_err(|_| AppError::IoFailed)?;
            let is_quit = line == "quit";

            tx.blocking_send(line).map_err(|_| AppError::IoFailed)?;

            if is_quit {
                QUIT_FLAG.store(true, Ordering::Relaxed);
                break;
            }
        }
        Ok(())
    });

    let rx_handle = tokio::spawn(async move {
        // Returns None once tx handle is dropped by the sender thread
        while let Some(line) = rx.recv().await {
            println!("{line}");
        }
    });

    while !QUIT_FLAG.load(Ordering::Relaxed) {
        let (_stream, addr) = listener.accept().await.map_err(|_| {
            eprintln!("Unable to accept client connection");
            AppError::ClientConnectFailed
        })?;

        println!("Client connected: {addr}");
    }

    tx_handle.join().map_err(|_| AppError::JoinFailed)??;
    rx_handle.await.map_err(|_| AppError::JoinFailed)?;

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => ExitCode::from(e),
    }
}
```
Ran five times:
```shell
CSE 5402/studio18/rustasyncclient on  main [!⇡] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/rustasyncclient`
Connected to server
Sent: Hello from async client :)
```

Server:
```shell
CSE 5402/studio20/reactive_server on  main [!⇡] is 📦 v0.1.0 via 🦀 v1.71.0
❯ cargo run 7777
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/reactive_server 7777`
Listening on 127.0.0.1:7777
Client connected: 127.0.0.1:65063
Client connected: 127.0.0.1:65067
Client connected: 127.0.0.1:65068
Client connected: 127.0.0.1:65069
quit
quit
Client connected: 127.0.0.1:65072

CSE 5402/studio20/reactive_server on  main [!⇡] is 📦 v0.1.0 via 🦀 v1.71.0
```
