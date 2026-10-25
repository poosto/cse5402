// order.rs
// The order type that moves through the restaurant, and its status.

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ORDER_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Ordered,
    Ready,
    Served,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            OrderStatus::Ordered => "ORDERED",
            OrderStatus::Ready => "READY",
            OrderStatus::Served => "SERVED",
        };
        write!(f, "{:<7}", label)
    }
}

#[derive(Debug)]
pub struct Order {
    pub id: usize,
    pub table: usize,
    pub customers: usize,
    // The waiter who took the order at the table.
    pub waiter: usize,
    pub status: OrderStatus,
}

impl Order {
    pub fn new(table: usize, customers: usize, waiter: usize) -> Self {
        Order {
            id: NEXT_ORDER_ID.fetch_add(1, Ordering::SeqCst),
            table,
            customers,
            waiter,
            status: OrderStatus::Ordered,
        }
    }

    // Moves the order to a new status and prints the change.
    pub fn set_status(&mut self, status: OrderStatus, note: &str) {
        self.status = status;
        self.announce(note);
    }

    pub fn announce(&self, note: &str) {
        println!(
            "[{}] order #{:<2} table {:<2} ({} customers, Waiter {}'s table) {}",
            self.status, self.id, self.table, self.customers, self.waiter, note
        );
    }
}
