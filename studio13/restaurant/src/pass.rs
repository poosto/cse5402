// pass.rs
// The pass is the counter where the chef sets finished plates.
// Waiters pick food up from it with the Leader/Followers pattern.
//
// Only one waiter, the leader, waits on the pass at a time. The rest are
// followers waiting for their turn to lead. When the leader gets a plate,
// it first promotes a follower to be the new leader and then carries the
// plate out. So a waiter is always watching the pass while others serve.

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

use crate::order::Order;

struct PassState {
    plates: VecDeque<Order>,
    has_leader: bool,
    closed: bool,
}

pub struct Pass {
    state: Mutex<PassState>,
    plate_ready: Condvar,
    leader_free: Condvar,
}

impl Pass {
    pub fn new() -> Self {
        Pass {
            state: Mutex::new(PassState {
                plates: VecDeque::new(),
                has_leader: false,
                closed: false,
            }),
            plate_ready: Condvar::new(),
            leader_free: Condvar::new(),
        }
    }

    // Called by the chef side when a plate is done.
    pub fn put(&self, order: Order) {
        let mut state = self.state.lock().unwrap();
        state.plates.push_back(order);
        self.plate_ready.notify_one();
    }

    // Called by the chef side once there will be no more plates.
    pub fn close(&self) {
        let mut state = self.state.lock().unwrap();
        state.closed = true;
        self.plate_ready.notify_all();
    }

    // A waiter joins the pool and serves plates until the kitchen closes.
    // Returns how many plates this waiter carried out.
    pub fn join_pool<F: FnMut(Order)>(&self, waiter: usize, mut serve: F) -> usize {
        let mut served = 0;
        loop {
            let mut state = self.state.lock().unwrap();

            // Follower: wait until there is no leader.
            while state.has_leader {
                state = self.leader_free.wait(state).unwrap();
            }
            state.has_leader = true;
            println!("    Waiter {} is now the leader, watching the pass", waiter);

            // Leader: wait for a plate or for the kitchen to close.
            while state.plates.is_empty() && !state.closed {
                state = self.plate_ready.wait(state).unwrap();
            }
            let plate = state.plates.pop_front();

            // Step down and promote a follower before doing the work.
            state.has_leader = false;
            self.leader_free.notify_one();
            drop(state);

            match plate {
                Some(order) => {
                    println!(
                        "    Waiter {} picks up order #{} and hands off leadership",
                        waiter, order.id
                    );
                    serve(order);
                    served += 1;
                }
                // Closed and empty. The notify above lets the next
                // follower see this too, so the whole pool shuts down.
                None => return served,
            }
        }
    }
}
