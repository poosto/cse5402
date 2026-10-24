use rand::prelude::SliceRandom;
use rand::Rng;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc;
use std::{
    sync::{Arc, Mutex},
    thread,
};

struct Table {
    number: usize,
    customers: Vec<Customer>,
}

impl Table {
    fn new(number: usize) -> Self {
        Table {
            number,
            customers: Vec::new(),
        }
    }

    fn seat(&mut self, seat_number: usize) {
        let customer = Customer::new(self.number, seat_number);
        self.customers.push(customer);
    }
}

#[derive(Default)]
struct Waiter {
    handle: Option<thread::JoinHandle<()>>,
    tables: Vec<Table>,
}

trait WaiterHandle {
    fn activate(&self, restaurant: Arc<Restaurant>);
    fn finish(&self) -> thread::Result<()>;
}

impl WaiterHandle for Arc<Mutex<Waiter>> {
    fn activate(&self, restaurant: Arc<Restaurant>) {
        let mut guard = self.lock().expect("Poisoned mutex");

        // Create an Arc clone to move into the handle
        let thread_waiter = self.clone();

        if guard.handle.is_none() {
            guard.handle = Some(thread::spawn(move || {
                let guard = thread_waiter.lock().expect("Poisoned mutex");

                println!("Welcome to AO's place!");

                guard.tables.iter().for_each(|table| {
                    table.customers.iter().for_each(|customer| {
                        customer.order(&guard);
                    });
                });

                guard.tables.iter().for_each(|table| {
                    table.customers.iter().for_each(|customer| {
                        customer
                            .order_items
                            .borrow_mut()
                            // drain(..) steals all orders from the
                            // customer and sends them to the chef
                            .drain(..)
                            .for_each(|item| {
                                restaurant.chef.lock().expect("Poisoned mutex").order(item);
                            });
                    });
                });
            }))
        }
    }

    fn finish(&self) -> thread::Result<()> {
        // Lock dropped at end of statement (;) since it's a temporary
        // so there's no deadlock risk when calling join() while holding the mutex
        //
        // handle.take() moves the
        // value out of the Option and reaplces it with None.
        // This leaves Waiter in a valid state, and doesn't require moving it
        // (which we can't do from a Mutex).
        let handle = self.lock().expect("Poisoned mutex").handle.take();

        match handle {
            Some(handle) => handle.join(),
            None => Ok(()),
        }
    }
}

impl Waiter {
    fn new() -> Self {
        Self::default()
    }

    fn assign(&mut self, table: Table) {
        self.tables.push(table);
    }

    fn take_order(&self, order: OrderItem) {
        let customer = self
            .tables
            .iter()
            .find(|table| table.number == order.table_number)
            .map(|table| &table.customers)
            // and_then flattens nested options, where map doesn't do any flattening
            .and_then(|customers| {
                customers
                    .iter()
                    .find(|customer| customer.seat_number == order.seat_number)
            });

        if let Some(customer) = customer {
            customer.order_items.borrow_mut().push(order);
        }
    }
}

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
    // no reflection :(
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

    fn random() -> Food {
        *Food::ALL.choose(&mut rand::thread_rng()).unwrap()
    }
}

#[derive(Debug)]
enum Status {
    Ordered,
    Ready,
    Served,
}

#[derive(Debug)]
struct OrderItem {
    table_number: usize,
    seat_number: usize,
    food_order: Food,
    order_status: Status,
}

impl OrderItem {
    fn new(table_number: usize, seat_number: usize, food_order: Food) -> Self {
        Self {
            table_number,
            seat_number,
            food_order,
            order_status: Status::Ordered,
        }
    }
}

struct Customer {
    table_number: usize,
    seat_number: usize,
    // Interior mutability so that we don't need multiple mutable borrows
    // This is particularly because of Customer::order, which needed a &mut Waiter,
    // but to get the Customer out, you needed to borrow Waiter.
    order_items: RefCell<Vec<OrderItem>>,
}

impl Customer {
    fn new(table_number: usize, seat_number: usize) -> Self {
        Self {
            table_number,
            seat_number,
            order_items: RefCell::new(Vec::new()),
        }
    }

    fn order(&self, waiter: &Waiter) {
        let item = OrderItem::new(self.table_number, self.seat_number, Food::random());
        waiter.take_order(item);
    }
}

struct Chef {
    handle: Option<thread::JoinHandle<()>>,
    tx: mpsc::Sender<WorkItem>,
    rx: Option<mpsc::Receiver<WorkItem>>,
}

impl Chef {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            handle: None,
            tx,
            rx: Some(rx),
        }
    }

    fn order(&self, order_item: OrderItem) {
        let work_item = WorkItem::Process(order_item);
        self.tx.send(work_item).unwrap();
    }

    fn quit(&self) {
        self.tx.send(WorkItem::Quit).unwrap();
    }

    fn activate(&mut self) {
        if self.handle.is_none() {
            let rx = self.rx.take().unwrap();

            self.handle = Some(thread::spawn(move || {
                println!("Ready for orders!");

                for item in rx {
                    match item {
                        WorkItem::Quit => return,
                        WorkItem::Process(mut order_item) => {
                            order_item.order_status = Status::Ready;
                            println!("{:?}", order_item);
                        }
                    };
                }
            }))
        }
    }

    fn finish(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.join().expect("Unable to join thread");
        }
    }
}

enum WorkItem {
    Quit,
    Process(OrderItem),
}

struct Restaurant {
    chef: Arc<Mutex<Chef>>,
    waiters: Vec<Arc<Mutex<Waiter>>>,
    table_to_waiter: HashMap<usize, Arc<Mutex<Waiter>>>,
    leader_queue: Mutex<VecDeque<Arc<Mutex<Waiter>>>>,
}

impl Restaurant {
    fn new(chef: Arc<Mutex<Chef>>, waiters: Vec<Arc<Mutex<Waiter>>>) -> Self {
        let mut table_to_waiter = HashMap::new();

        for w in &waiters {
            for t in &w.lock().expect("Poisoned mutex").tables {
                table_to_waiter.insert(t.number, Arc::clone(w));
            }
        }

        Self {
            chef,
            waiters,
            table_to_waiter,
            leader_queue: Mutex::new(VecDeque::new()),
        }
    }
}

fn main() {
    const NUM_WAITERS: usize = 4;
    const NUM_TABLES_PER_WAITER: usize = 5;

    let mut waiters: [Waiter; NUM_WAITERS] = std::array::from_fn(|_| Waiter::new());

    for w in 0..NUM_WAITERS {
        for tid in 0..NUM_TABLES_PER_WAITER {
            let mut table = Table::new(w * NUM_TABLES_PER_WAITER + tid);
            for i in 0..rand::thread_rng().gen_range(2..=6) {
                table.seat(i);
            }
            waiters[w].assign(table);
        }
    }

    let arc_waiters: Vec<Arc<Mutex<Waiter>>> = waiters
        .map(|waiter| Arc::new(Mutex::new(waiter)))
        .into_iter()
        .collect();

    let chef = Arc::new(Mutex::new(Chef::new()));
    let restaurant = Arc::new(Restaurant::new(chef, arc_waiters));

    println!("Closing time!");
}
