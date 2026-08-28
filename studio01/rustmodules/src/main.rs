#[cfg(oldexercise)] pub const VAL: usize = 42;
#[cfg(oldexercise)] pub static VAL: usize = 42;
#[cfg(oldexercise)] pub static mut VAL: usize = 42;

mod foo {
    use std::sync::atomic::AtomicUsize;
    pub static VAL: AtomicUsize = AtomicUsize::new(42);
}

use foo::VAL;
use std::sync::atomic::Ordering;

fn main() {
    #[cfg(oldexercise)] unsafe {
        println!("Hello, world! {}", VAL);
    }

    println!("Value: {}", VAL.load(Ordering::SeqCst));
    VAL.store(33, Ordering::SeqCst);
    println!("New value: {}", VAL.load(Ordering::SeqCst));
}

