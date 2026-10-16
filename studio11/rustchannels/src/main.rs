use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn send_element<T>(tx: &mpsc::Sender<T>, element: T) {
    tx.send(element).expect("Failed to send to channel");
    thread::yield_now();
}

fn send_elements<T: Clone>(tx: &mpsc::Sender<T>, elements: &[T]) {
    elements.iter().for_each(|e| send_element(tx, e.to_owned()));
}

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let rx = Arc::new(Mutex::new(rx));

    let handles: Vec<_> = (0..=3)
        .map(|idx| {
            let rx = rx.clone();
            let prefix = if idx % 2 == 0 { "[A]" } else { "[B]" };

            thread::spawn(move || {
                let guard = rx.lock().expect("Unable to acquire mutex");

                loop {
                    match guard.recv() {
                        Ok(msg) if msg.starts_with(prefix) => println!("{msg}"),
                        Ok(msg) => println!("[delegated] {msg}"),
                        Err(_) => break,
                    }
                }
            })
        })
        .collect();

    let words = vec![
        String::from("[A] hello"),
        String::from("[A] there"),
        String::from("[A] how"),
        String::from("[A] are you?"),
    ];

    for idx in 0..=3 {
        let tx = tx.clone();
        thread::spawn(move || {
            let prefix = if idx < 1 { "[A]" } else { "[B]" };
            send_element(&tx, format!("{prefix} Hello from Thread {idx}"));
            send_element(&tx, format!("{prefix} Hello AGAIN from Thread {idx}"));
        });
    }

    send_elements(&tx, &words);
    drop(tx);

    handles
        .into_iter()
        .for_each(|t| t.join().expect("Thread failed to join"));
}
