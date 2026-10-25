// main.rs
// Studio 13: a restaurant that uses the Active Object and
// Half-Sync/Half-Async patterns.
// Each waiter and the chef is an active object with its own thread.
// The waiters take orders from the customers at their tables and drop
// them off to the chef in an MPSC channel.
// The chef takes each order out of the channel, marks it ready, and prints it.

use rand::seq::SliceRandom;
use rand::Rng;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

const NUM_WAITERS: usize = 4;
const TABLES_PER_WAITER: usize = 5;
const MIN_CUSTOMERS: usize = 2;
const MAX_CUSTOMERS: usize = 6;

// The kinds of food a customer can order
#[derive(Debug, Clone, Copy)]
enum Food {
    Chicken,
    Fish,
    Noodles,
    Spinach,
    Carrots,
    Asparagus,
    Yams,
    Salad,
    Coleslaw,
}

impl Food {
    const ALL: [Food; 9] = [
        Food::Chicken,
        Food::Fish,
        Food::Noodles,
        Food::Spinach,
        Food::Carrots,
        Food::Asparagus,
        Food::Yams,
        Food::Salad,
        Food::Coleslaw,
    ];

    // Picks a random kind of food
    fn random() -> Food {
        *Food::ALL
            .choose(&mut rand::thread_rng())
            .expect("Food list is empty")
    }
}

// The status of an order item
// Served is not used until the Leader/Followers exercise
#[allow(dead_code)]
#[derive(Debug)]
enum Status {
    Ordered,
    Ready,
    Served,
}

// One item that a customer ordered
#[derive(Debug)]
struct OrderItem {
    table_number: usize,
    seat_number: usize,
    food: Food,
    status: Status,
}

impl OrderItem {
    // Makes a new order item with the status set to ordered
    fn new(table_number: usize, seat_number: usize, food: Food) -> Self {
        OrderItem {
            table_number,
            seat_number,
            food,
            status: Status::Ordered,
        }
    }
}

// A customer sitting at a seat at a table
struct Customer {
    table_number: usize,
    seat_number: usize,
    // A mutex lets the waiter add items while the waiter is shared between threads
    order_items: Mutex<Vec<OrderItem>>,
}

impl Customer {
    // Makes a new customer with no order items
    fn new(table_number: usize, seat_number: usize) -> Self {
        Customer {
            table_number,
            seat_number,
            order_items: Mutex::new(Vec::new()),
        }
    }

    // Orders a random food item from the given waiter
    fn order(&self, waiter: &Waiter) {
        let item = OrderItem::new(self.table_number, self.seat_number, Food::random());
        waiter.take_order(item);
    }
}

// A table with a number and the customers sitting at it
struct Table {
    table_number: usize,
    customers: Vec<Customer>,
}

impl Table {
    // Makes a new empty table with the given number
    fn new(table_number: usize) -> Self {
        Table {
            table_number,
            customers: Vec::new(),
        }
    }

    // Seats a new customer at the given seat number
    fn seat(&mut self, seat_number: usize) {
        self.customers
            .push(Customer::new(self.table_number, seat_number));
    }
}

// The work that is sent through a channel
enum WorkItem {
    Quit,
    Process(OrderItem),
}

// The chef is an active object. It cooks orders on its own thread.
struct Chef {
    // The mutex lets activate and finish change the handle through an Arc
    handle: Mutex<Option<thread::JoinHandle<()>>>,
    // Sender is not Sync in Rust 1.71, so it needs a mutex to be shared
    sender: Mutex<Sender<WorkItem>>,
    // activate moves the receiver into the chef's thread
    receiver: Mutex<Option<Receiver<WorkItem>>>,
}

impl Chef {
    // Makes a new chef with a new channel and no thread
    fn new() -> Self {
        let (sender, receiver) = channel();
        Chef {
            handle: Mutex::new(None),
            sender: Mutex::new(sender),
            receiver: Mutex::new(Some(receiver)),
        }
    }

    // Spawns the chef's thread and stores its handle
    fn activate(&self) {
        let receiver = self.receiver.lock().expect("Poisoned mutex").take();
        if let Some(receiver) = receiver {
            let handle = thread::spawn(move || Chef::run(receiver));
            *self.handle.lock().expect("Poisoned mutex") = Some(handle);
        }
    }

    // The work done on the chef's thread
    fn run(receiver: Receiver<WorkItem>) {
        println!("Ready for orders!");

        // Take the next work item until quit is received
        // The loop also stops if every sender is gone
        while let Ok(work_item) = receiver.recv() {
            match work_item {
                WorkItem::Quit => break,
                WorkItem::Process(mut item) => {
                    item.status = Status::Ready;
                    println!(
                        "table: {}, seat: {}, food: {:?}, status: {:?}",
                        item.table_number, item.seat_number, item.food, item.status
                    );
                }
            }
        }
    }

    // Sends an order item to the chef's thread
    fn order(&self, item: OrderItem) {
        self.send(WorkItem::Process(item));
    }

    // Tells the chef's thread to stop
    fn quit(&self) {
        self.send(WorkItem::Quit);
    }

    // Moves a work item into the chef's channel
    fn send(&self, work_item: WorkItem) {
        let sender = self.sender.lock().expect("Poisoned mutex");
        if sender.send(work_item).is_err() {
            eprintln!("The chef's thread has stopped");
        }
    }

    // Joins with the chef's thread
    fn finish(&self) {
        // take() moves the handle out so the lock is not held during the join
        let handle = self.handle.lock().expect("Poisoned mutex").take();
        if let Some(handle) = handle {
            if handle.join().is_err() {
                eprintln!("The chef thread panicked");
            }
        }
    }
}

// A waiter is an active object. It runs its work on its own thread.
#[derive(Default)]
struct Waiter {
    // The mutex lets activate and finish change the handle through an Arc
    handle: Mutex<Option<thread::JoinHandle<()>>>,
    tables: Vec<Table>,
}

impl Waiter {
    // Makes a new waiter with no tables and no thread
    fn new() -> Self {
        Self::default()
    }

    // Moves a table into this waiter's list of tables
    fn assign(&mut self, table: Table) {
        self.tables.push(table);
    }

    // Moves an order item into the matching customer's list of order items
    fn take_order(&self, item: OrderItem) {
        let customer = self
            .tables
            .iter()
            .find(|table| table.table_number == item.table_number)
            .and_then(|table| {
                table
                    .customers
                    .iter()
                    .find(|customer| customer.seat_number == item.seat_number)
            });

        match customer {
            Some(customer) => customer
                .order_items
                .lock()
                .expect("Poisoned mutex")
                .push(item),
            None => eprintln!("No customer for {:?}", item),
        }
    }

    // Spawns this waiter's thread and stores its handle
    // The thread gets its own Arc to the chef
    fn activate(self: &Arc<Self>, chef: &Arc<Chef>) {
        let waiter = Arc::clone(self);
        let chef = Arc::clone(chef);
        let handle = thread::spawn(move || waiter.run(&chef));
        *self.handle.lock().expect("Poisoned mutex") = Some(handle);
    }

    // The work done on the waiter's thread
    fn run(&self, chef: &Chef) {
        println!("Welcome to AO's place!");

        // Take an order from every customer at every table
        for table in &self.tables {
            for customer in &table.customers {
                customer.order(self);
            }
        }

        // Drop off every order item to the chef
        for table in &self.tables {
            for customer in &table.customers {
                let mut order_items = customer.order_items.lock().expect("Poisoned mutex");
                for item in order_items.drain(..) {
                    chef.order(item);
                }
            }
        }
    }

    // Joins with this waiter's thread
    fn finish(&self) {
        // take() moves the handle out so the lock is not held during the join
        let handle = self.handle.lock().expect("Poisoned mutex").take();
        if let Some(handle) = handle {
            if handle.join().is_err() {
                eprintln!("A waiter thread panicked");
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    // Make four waiters
    let mut waiters: Vec<Waiter> = (0..NUM_WAITERS).map(|_| Waiter::new()).collect();

    // Make 20 tables with unique numbers, seat customers, and give each waiter five
    for (w, waiter) in waiters.iter_mut().enumerate() {
        for t in 0..TABLES_PER_WAITER {
            let mut table = Table::new(w * TABLES_PER_WAITER + t + 1);
            for seat in 1..=rng.gen_range(MIN_CUSTOMERS..=MAX_CUSTOMERS) {
                table.seat(seat);
            }
            waiter.assign(table);
        }
    }

    // Put each waiter in an Arc
    let waiters: Vec<Arc<Waiter>> = waiters.into_iter().map(Arc::new).collect();

    // Make the chef that all waiters can access, and start it first
    let chef = Arc::new(Chef::new());
    chef.activate();

    for waiter in &waiters {
        waiter.activate(&chef);
    }

    for waiter in &waiters {
        waiter.finish();
    }

    chef.quit();
    chef.finish();

    println!("Closing time!");
}
