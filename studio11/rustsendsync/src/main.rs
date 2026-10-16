use std::sync::Arc;
use std::thread;

fn main() {
    let phrase = String::from("hello from the child thread!");
    let arc = Arc::new(phrase.clone());

    let handle = thread::spawn(move || {
        println!("{arc}");
    });

    handle.join().expect("Thread failed to join");
}
