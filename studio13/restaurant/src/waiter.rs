// waiter.rs
// A waiter takes orders from their tables and sends them to the kitchen
// through the proxy. This is the async half of Half-Sync/Half-Async, since
// the waiter never waits for the chef. After that the waiter joins the
// Leader/Followers pool at the pass to bring food out.

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::kitchen::KitchenProxy;
use crate::order::{Order, OrderStatus};
use crate::pass::Pass;
use crate::rng::Rng;

pub struct Table {
    pub number: usize,
    pub customers: usize,
}

pub struct Waiter {
    pub id: usize,
    pub tables: Vec<Table>,
}

impl Waiter {
    // Returns how many plates this waiter served.
    pub fn work(self, kitchen: KitchenProxy, pass: Arc<Pass>) -> usize {
        let mut rng = Rng::new(self.id as u64 + 100);

        for table in &self.tables {
            // Walk over to the table and take the order.
            thread::sleep(Duration::from_millis(rng.range(30, 120)));
            let order = Order::new(table.number, table.customers, self.id);
            order.announce(&format!("taken by Waiter {}", self.id));
            kitchen.place_order(order);
        }

        // Let go of the proxy so the kitchen can close once every waiter
        // is done placing orders.
        drop(kitchen);
        println!("    Waiter {} has placed all orders and joins the pool", self.id);

        let id = self.id;
        pass.join_pool(id, |mut order| {
            // Carry the plate out to the table.
            thread::sleep(Duration::from_millis(rng.range(40, 100)));
            order.set_status(OrderStatus::Served, &format!("brought out by Waiter {}", id));
        })
    }
}
