// main.rs
// Studio 13: a restaurant that uses the Active Object,
// Half-Sync/Half-Async, and Leader/Followers patterns.
// Each waiter and the chef is an active object with its own thread.
// The waiters take orders from the customers at their tables and drop
// them off to the chef in an MPSC channel.
// Several chefs share the receiving end of that channel, so they cook at the
// same time. A chef takes each order out of the channel, marks it ready, and
// gives it to the waiter at the head of the leader queue.
// If the item is for one of that waiter's tables, the waiter moves to the end
// of the queue and serves it. Otherwise it passes the item to the right waiter.

use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const NUM_CHEFS: usize = 3;
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

// Holds the chefs, the waiters, and the leader queue
struct Restaurant {
    chefs: Vec<Arc<Chef>>,
    // How many chef threads are still running
    active_chefs: AtomicUsize,
    waiters: Vec<Arc<Waiter>>,
    // Finds the waiter for each table number
    table_waiters: HashMap<usize, Arc<Waiter>>,
    // The waiters who are in line to be the leader
    leader_queue: Mutex<VecDeque<Arc<Waiter>>>,
    // Wakes up threads that wait for the leader queue to change
    queue_changed: Condvar,
}

impl Restaurant {
    // Makes a new restaurant and maps each table number to its waiter
    fn new(chefs: Vec<Arc<Chef>>, waiters: Vec<Arc<Waiter>>) -> Self {
        let mut table_waiters = HashMap::new();
        for waiter in &waiters {
            for table in &waiter.tables {
                table_waiters.insert(table.table_number, Arc::clone(waiter));
            }
        }

        Restaurant {
            active_chefs: AtomicUsize::new(chefs.len()),
            chefs,
            waiters,
            table_waiters,
            leader_queue: Mutex::new(VecDeque::new()),
            queue_changed: Condvar::new(),
        }
    }

    // Puts a waiter at the end of the leader queue
    // If the waiter is already in the queue it is moved to the end
    fn enqueue(&self, waiter: &Arc<Waiter>) {
        let mut queue = self.leader_queue.lock().expect("Poisoned mutex");
        queue.retain(|w| !Arc::ptr_eq(w, waiter));
        queue.push_back(Arc::clone(waiter));
        self.queue_changed.notify_all();
    }

    // Returns the waiter at the head of the leader queue
    // Waits until the queue has at least one waiter
    fn leader(&self) -> Arc<Waiter> {
        let queue = self.leader_queue.lock().expect("Poisoned mutex");
        let queue = self
            .queue_changed
            .wait_while(queue, |queue| queue.is_empty())
            .expect("Poisoned mutex");
        Arc::clone(&queue[0])
    }

    // Waits until every waiter is in the leader queue
    fn wait_for_all_waiters(&self) {
        let queue = self.leader_queue.lock().expect("Poisoned mutex");
        let _queue = self
            .queue_changed
            .wait_while(queue, |queue| queue.len() < self.waiters.len())
            .expect("Poisoned mutex");
    }
}

// A chef is an active object. It cooks orders on its own thread.
// All chefs share one channel.
struct Chef {
    // The mutex lets activate and finish change the handle through an Arc
    handle: Mutex<Option<thread::JoinHandle<()>>>,
    // Sender is not Sync in Rust 1.71, so it needs a mutex to be shared
    sender: Mutex<Sender<WorkItem>>,
    // The receiving end is shared by all chefs
    // Only one chef at a time can take a work item out of it
    receiver: Arc<Mutex<Receiver<WorkItem>>>,
}

impl Chef {
    // Makes a new chef that uses the given shared channel and has no thread
    fn new(sender: Sender<WorkItem>, receiver: Arc<Mutex<Receiver<WorkItem>>>) -> Self {
        Chef {
            handle: Mutex::new(None),
            sender: Mutex::new(sender),
            receiver,
        }
    }

    // Spawns the chef's thread and stores its handle
    // The thread gets its own Arc to the receiver and to the restaurant
    fn activate(&self, restaurant: &Arc<Restaurant>) {
        let receiver = Arc::clone(&self.receiver);
        let restaurant = Arc::clone(restaurant);
        let handle = thread::spawn(move || Chef::run(&receiver, &restaurant));
        *self.handle.lock().expect("Poisoned mutex") = Some(handle);
    }

    // The work done on the chef's thread
    fn run(receiver: &Mutex<Receiver<WorkItem>>, restaurant: &Restaurant) {
        println!("Ready for orders!");

        // Take the next work item until quit is received
        // The loop also stops if every sender is gone
        loop {
            // The lock is let go at the end of this statement,
            // so other chefs can take items while this chef cooks
            let work_item = match receiver.lock().expect("Poisoned mutex").recv() {
                Ok(work_item) => work_item,
                Err(_) => break,
            };

            match work_item {
                WorkItem::Quit => break,
                WorkItem::Process(mut item) => {
                    // Mark the item ready and give it to the leader
                    item.status = Status::Ready;
                    restaurant.leader().pickup(WorkItem::Process(item));
                }
            }
        }

        // The last chef to stop tells every waiter to quit
        // Earlier chefs do not, since other chefs may still give items to waiters
        if restaurant.active_chefs.fetch_sub(1, Ordering::SeqCst) == 1 {
            for waiter in &restaurant.waiters {
                waiter.quit();
            }
        }
    }

    // Sends an order item to the chef's thread
    fn order(&self, item: OrderItem) {
        self.send(WorkItem::Process(item));
    }

    // Sends a quit into the shared channel
    // One chef takes it and stops
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
struct Waiter {
    // The mutex lets activate and finish change the handle through an Arc
    handle: Mutex<Option<thread::JoinHandle<()>>>,
    tables: Vec<Table>,
    // Sender is not Sync in Rust 1.71, so it needs a mutex to be shared
    sender: Mutex<Sender<WorkItem>>,
    // activate moves the receiver into the waiter's thread
    receiver: Mutex<Option<Receiver<WorkItem>>>,
}

impl Waiter {
    // Makes a new waiter with no tables, no thread, and a new channel
    fn new() -> Self {
        let (sender, receiver) = channel();
        Waiter {
            handle: Mutex::new(None),
            tables: Vec::new(),
            sender: Mutex::new(sender),
            receiver: Mutex::new(Some(receiver)),
        }
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

    // Moves a work item into this waiter's channel
    fn pickup(&self, work_item: WorkItem) {
        let sender = self.sender.lock().expect("Poisoned mutex");
        if sender.send(work_item).is_err() {
            eprintln!("A waiter's thread has stopped");
        }
    }

    // Tells this waiter's thread to stop
    fn quit(&self) {
        self.pickup(WorkItem::Quit);
    }

    // Spawns this waiter's thread and stores its handle
    // The thread gets its own Arc to the restaurant
    fn activate(self: &Arc<Self>, restaurant: &Arc<Restaurant>) {
        let receiver = self.receiver.lock().expect("Poisoned mutex").take();
        if let Some(receiver) = receiver {
            let waiter = Arc::clone(self);
            let restaurant = Arc::clone(restaurant);
            let handle = thread::spawn(move || waiter.run(receiver, &restaurant));
            *self.handle.lock().expect("Poisoned mutex") = Some(handle);
        }
    }

    // The work done on the waiter's thread
    fn run(self: &Arc<Self>, receiver: Receiver<WorkItem>, restaurant: &Restaurant) {
        println!("Welcome to AO's place!");

        // Take an order from every customer at every table
        for table in &self.tables {
            for customer in &table.customers {
                customer.order(self);
            }
        }

        // Drop off every order item to the chef
        let mut ordered = 0;
        for table in &self.tables {
            for customer in &table.customers {
                let mut order_items = customer.order_items.lock().expect("Poisoned mutex");
                for item in order_items.drain(..) {
                    // All chefs share one channel, so any chef can take the order
                    restaurant.chefs[0].order(item);
                    ordered += 1;
                }
            }
        }

        // All orders are with the chef, so get in line to be the leader
        restaurant.enqueue(self);

        // Take work items until quit is received and every item is served
        let mut served = 0;
        let mut quit = false;
        while !quit || served < ordered {
            let work_item = match receiver.recv() {
                Ok(work_item) => work_item,
                Err(_) => break,
            };

            match work_item {
                WorkItem::Quit => quit = true,
                WorkItem::Process(mut item) => {
                    match restaurant.table_waiters.get(&item.table_number) {
                        // The item is for one of this waiter's own tables
                        Some(owner) if Arc::ptr_eq(owner, self) => {
                            restaurant.enqueue(self);
                            item.status = Status::Served;
                            println!(
                                "table: {}, seat: {}, food: {:?}, status: {:?}",
                                item.table_number, item.seat_number, item.food, item.status
                            );
                            served += 1;
                        }
                        // The item is for another waiter's table
                        Some(owner) => owner.pickup(WorkItem::Process(item)),
                        None => eprintln!("No waiter for table {}", item.table_number),
                    }
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

    // Make several chefs that share one channel
    let (sender, receiver) = channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let chefs: Vec<Arc<Chef>> = (0..NUM_CHEFS)
        .map(|_| Arc::new(Chef::new(sender.clone(), Arc::clone(&receiver))))
        .collect();

    // Make the restaurant with the chefs and the waiters
    let restaurant = Arc::new(Restaurant::new(chefs, waiters));

    // Start the chefs first, then the waiters
    for chef in &restaurant.chefs {
        chef.activate(&restaurant);
    }
    for waiter in &restaurant.waiters {
        waiter.activate(&restaurant);
    }

    // Once every waiter has given all its orders to the chefs, the chefs can quit
    // Each chef takes one quit from the shared channel
    restaurant.wait_for_all_waiters();
    for chef in &restaurant.chefs {
        chef.quit();
    }
    for chef in &restaurant.chefs {
        chef.finish();
    }

    for waiter in &restaurant.waiters {
        waiter.finish();
    }

    println!("Closing time!");
}
