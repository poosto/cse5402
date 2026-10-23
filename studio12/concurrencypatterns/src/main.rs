use std::{
    sync::{Arc, Mutex},
    thread,
};

struct Table {
    number: usize,
}

impl Table {
    fn new(number: usize) -> Self {
        Table { number }
    }
}

#[derive(Default)]
struct Waiter {
    handle: Option<thread::JoinHandle<()>>,
    tables: Vec<Table>,
}

impl Waiter {
    fn new() -> Self {
        Self::default()
    }

    fn assign(&mut self, table: Table) {
        self.tables.push(table);
    }

    fn activate(&mut self) {
        if self.handle.is_none() {
            self.handle = Some(thread::spawn(|| {
                println!("Welcome to AO's place!");
            }))
        }
    }

    fn finish(&mut self) -> thread::Result<()> {
        // only have &mut self, so self.handle.take() moves the
        // value out of the Option and reaplces it with None.
        // This leaves Waiter in a valid state.
        if let Some(handle) = self.handle.take() {
            handle.join()
        } else {
            Ok(())
        }
    }
}

fn main() {
    const NUM_WAITERS: usize = 4;
    const NUM_TABLES_PER_WAITER: usize = 5;

    let mut waiters: [Waiter; NUM_WAITERS] = std::array::from_fn(|_| Waiter::new());

    for w in 0..NUM_WAITERS {
        for tid in 0..NUM_TABLES_PER_WAITER {
            waiters[w].assign(Table::new(w * NUM_TABLES_PER_WAITER + tid));
        }
    }

    let arc_waiters: [Arc<Mutex<Waiter>>; NUM_WAITERS] =
        waiters.map(|waiter| Arc::new(Mutex::new(waiter)));

    arc_waiters.iter().for_each(|aw| {
        aw.lock().expect("Poisoned mutex").activate();
    });

    arc_waiters.iter().for_each(|aw| {
        aw.lock()
            .expect("Poisoned mutex")
            .finish()
            .expect("Unable to join threads");
    });

    println!("Closing time!");
}
