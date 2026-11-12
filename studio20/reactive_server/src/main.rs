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
