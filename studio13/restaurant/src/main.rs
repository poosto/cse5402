// main.rs
// A restaurant that uses three concurrency patterns:
//   Active Object          : waiters talk to the kitchen through a proxy (kitchen.rs)
//   Half-Sync/Half-Async   : waiters place orders async, the chef cooks sync
//   Leader/Followers       : waiters take turns watching the pass for food (pass.rs)

mod kitchen;
mod order;
mod pass;
mod rng;
mod waiter;

use std::sync::Arc;
use std::thread;

use kitchen::{open_kitchen, Chef};
use pass::Pass;
use rng::Rng;
use waiter::{Table, Waiter};

const NUM_WAITERS: usize = 4;
const TABLES_PER_WAITER: usize = 5;
const MIN_CUSTOMERS: u64 = 2;
const MAX_CUSTOMERS: u64 = 6;

fn main() {
    let mut rng = Rng::new(0);
    let pass = Arc::new(Pass::new());
    let (kitchen, kitchen_thread) = open_kitchen(
        Chef {
            name: String::from("Chef"),
        },
        Arc::clone(&pass),
    );

    // Each waiter gets their own 5 tables, numbered so no two waiters share one.
    let mut waiters = Vec::new();
    for w in 1..=NUM_WAITERS {
        let tables: Vec<Table> = (1..=TABLES_PER_WAITER)
            .map(|t| Table {
                number: (w - 1) * TABLES_PER_WAITER + t,
                customers: rng.range(MIN_CUSTOMERS, MAX_CUSTOMERS) as usize,
            })
            .collect();
        let numbers: Vec<String> = tables
            .iter()
            .map(|t| format!("{}({})", t.number, t.customers))
            .collect();
        println!("Waiter {} has tables (customers): {}", w, numbers.join(", "));
        waiters.push(Waiter { id: w, tables });
    }
    println!();

    let handles: Vec<_> = waiters
        .into_iter()
        .map(|waiter| {
            let kitchen = kitchen.clone();
            let pass = Arc::clone(&pass);
            thread::spawn(move || waiter.work(kitchen, pass))
        })
        .collect();

    // Main does not place orders, so it lets go of its proxy right away.
    drop(kitchen);

    let mut total = 0;
    let mut counts = Vec::new();
    for (i, handle) in handles.into_iter().enumerate() {
        let served = handle.join().expect("a waiter thread panicked");
        counts.push(format!("Waiter {}: {}", i + 1, served));
        total += served;
    }
    kitchen_thread.join().expect("the kitchen thread panicked");

    println!();
    println!("Plates carried out: {}", counts.join(", "));
    println!(
        "Served {} of {} orders.",
        total,
        NUM_WAITERS * TABLES_PER_WAITER
    );
}
