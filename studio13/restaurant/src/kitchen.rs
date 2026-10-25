// kitchen.rs
// Active Object pattern plus the sync half of Half-Sync/Half-Async.
//
//   KitchenProxy  : the proxy. Waiters call place_order and return at once.
//   MethodRequest : a call turned into a message.
//   activation queue : the mpsc channel between the proxy and the scheduler.
//   scheduler     : the kitchen thread. It takes one request at a time off
//                   the queue and runs it on the servant.
//   Chef          : the servant. It cooks one order at a time, blocking
//                   while it cooks. This is the synchronous layer.
//
// When an order is cooked, the scheduler puts it on the pass, which is the
// result handoff back to the waiters.

use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::order::{Order, OrderStatus};
use crate::pass::Pass;

const COOK_MS_PER_CUSTOMER: u64 = 40;

enum MethodRequest {
    Cook(Order),
}

#[derive(Clone)]
pub struct KitchenProxy {
    activation_queue: Sender<MethodRequest>,
}

impl KitchenProxy {
    // Async call. Queues the order and returns without waiting for the chef.
    pub fn place_order(&self, order: Order) {
        self.activation_queue
            .send(MethodRequest::Cook(order))
            .expect("kitchen closed before all orders were placed");
    }
}

pub struct Chef {
    pub name: String,
}

impl Chef {
    // Sync work. The chef thread blocks until this order is done.
    fn cook(&self, mut order: Order) -> Order {
        println!(
            "        {} starts cooking order #{} for table {}",
            self.name, order.id, order.table
        );
        thread::sleep(Duration::from_millis(
            COOK_MS_PER_CUSTOMER * order.customers as u64,
        ));
        order.set_status(OrderStatus::Ready, &format!("cooked by {}", self.name));
        order
    }
}

// Starts the scheduler thread. It runs until every proxy has been dropped,
// then closes the pass so the waiters know no more food is coming.
pub fn open_kitchen(chef: Chef, pass: Arc<Pass>) -> (KitchenProxy, JoinHandle<()>) {
    let (tx, rx) = channel::<MethodRequest>();
    let scheduler = thread::spawn(move || {
        for request in rx {
            match request {
                MethodRequest::Cook(order) => {
                    let cooked = chef.cook(order);
                    pass.put(cooked);
                }
            }
        }
        println!("        {} has cooked every order. Kitchen closed.", chef.name);
        pass.close();
    });
    (KitchenProxy { activation_queue: tx }, scheduler)
}
