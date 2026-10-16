use std::env;
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

static FLAG: AtomicBool = AtomicBool::new(false);

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let handle = thread::spawn(move || -> io::Result<()> {
        loop {
            let (mut stream, _addr) = listener.accept()?;

            let mut text = String::new();
            stream.read_to_string(&mut text)?;

            if !text.is_empty() {
                println!("recv: {text}");
            }

            if FLAG.load(Ordering::Relaxed) {
                break Ok(());
            }
        }
    });

    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    stream.write_all(
        &env::args()
            .skip(1)
            .fold(String::new(), |mut accum, arg| {
                if !accum.is_empty() {
                    accum.push(' ');
                }

                accum.push_str(&arg);

                accum
            })
            .into_bytes(),
    )?;

    stream.shutdown(Shutdown::Write)?;
    FLAG.store(true, Ordering::Relaxed);
    TcpStream::connect("127.0.0.1:7878")?;

    handle.join().expect("Server thread panicked")?;

    Ok(())
}
