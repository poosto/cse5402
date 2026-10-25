### Studio 12

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Compile and run your program, and as the answer to this exercise please show the output that was produced.*
```shell
Welcome to AO's place!
Closing time!
```

3) *Compile and run your code, and as the answer to this question please show the output that was produced.*
```shell
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
Closing time!
```

4) *Compile and run your code, and as the answer to this question please show the output that was produced.* \
```rust
use rand::prelude::SliceRandom;
use rand::Rng;
use std::cell::RefCell;
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
    fn activate(&self);
    fn finish(&self) -> thread::Result<()>;
}

impl WaiterHandle for Arc<Mutex<Waiter>> {
    fn activate(&self) {
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
                        customer.order_items.borrow().iter().for_each(|item| {
                            println!("{:?}", item);
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

    let arc_waiters: [Arc<Mutex<Waiter>>; NUM_WAITERS] =
        waiters.map(|waiter| Arc::new(Mutex::new(waiter)));

    arc_waiters.iter().for_each(|aw| aw.activate());

    arc_waiters.iter().for_each(|aw| {
        aw.finish().expect("Unable to join threads");
    });

    println!("Closing time!");
}
```
```shell
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
OrderItem { table_number: 5, seat_number: 0, food_order: Spinach, order_status: Ordered }
OrderItem { table_number: 5, seat_number: 1, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 15, seat_number: 0, food_order: Chicken, order_status: Ordered }
OrderItem { table_number: 15, seat_number: 1, food_order: Chicken, order_status: Ordered }
OrderItem { table_number: 15, seat_number: 2, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 15, seat_number: 3, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 15, seat_number: 4, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 16, seat_number: 0, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 0, seat_number: 0, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 5, seat_number: 2, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 16, seat_number: 1, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 10, seat_number: 0, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 0, seat_number: 1, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 5, seat_number: 3, food_order: Spinach, order_status: Ordered }
OrderItem { table_number: 0, seat_number: 2, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 16, seat_number: 2, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 16, seat_number: 3, food_order: Chicken, order_status: Ordered }
OrderItem { table_number: 10, seat_number: 1, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 5, seat_number: 4, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 6, seat_number: 0, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 6, seat_number: 1, food_order: Chicken, order_status: Ordered }
OrderItem { table_number: 6, seat_number: 2, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 7, seat_number: 0, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 7, seat_number: 1, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 7, seat_number: 2, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 7, seat_number: 3, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 7, seat_number: 4, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 8, seat_number: 0, food_order: Chicken, order_status: Ordered }
OrderItem { table_number: 8, seat_number: 1, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 9, seat_number: 0, food_order: Spinach, order_status: Ordered }
OrderItem { table_number: 9, seat_number: 1, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 9, seat_number: 2, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 10, seat_number: 2, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 11, seat_number: 0, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 11, seat_number: 1, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 11, seat_number: 2, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 11, seat_number: 3, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 11, seat_number: 4, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 12, seat_number: 0, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 12, seat_number: 1, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 13, seat_number: 0, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 13, seat_number: 1, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 13, seat_number: 2, food_order: Chicken, order_status: Ordered }
OrderItem { table_number: 13, seat_number: 3, food_order: Spinach, order_status: Ordered }
OrderItem { table_number: 14, seat_number: 0, food_order: Spinach, order_status: Ordered }
OrderItem { table_number: 14, seat_number: 1, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 0, seat_number: 3, food_order: Chicken, order_status: Ordered }
OrderItem { table_number: 1, seat_number: 0, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 1, seat_number: 1, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 1, seat_number: 2, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 1, seat_number: 3, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 2, seat_number: 0, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 2, seat_number: 1, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 2, seat_number: 2, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 3, seat_number: 0, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 3, seat_number: 1, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 4, seat_number: 0, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 4, seat_number: 1, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 4, seat_number: 2, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 4, seat_number: 3, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 16, seat_number: 4, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 16, seat_number: 5, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 17, seat_number: 0, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 17, seat_number: 1, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 17, seat_number: 2, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 17, seat_number: 3, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 17, seat_number: 4, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 18, seat_number: 0, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 18, seat_number: 1, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 18, seat_number: 2, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 18, seat_number: 3, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 19, seat_number: 0, food_order: Spinach, order_status: Ordered }
OrderItem { table_number: 19, seat_number: 1, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 19, seat_number: 2, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 19, seat_number: 3, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 19, seat_number: 4, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 19, seat_number: 5, food_order: Yams, order_status: Ordered }
Closing time!
```

5) *Compile and run your code, and as the answer to this question please show the output that was produced by the chef
object.*
```shell
Ready for orders!
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
OrderItem { table_number: 10, seat_number: 0, food_order: Salad, order_status: Ordered }
Welcome to AO's place!
OrderItem { table_number: 5, seat_number: 0, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 5, seat_number: 1, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 5, seat_number: 2, food_order: Chicken, order_status: Ordered }
OrderItem { table_number: 5, seat_number: 3, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 6, seat_number: 0, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 6, seat_number: 1, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 6, seat_number: 2, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 6, seat_number: 3, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 6, seat_number: 4, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 7, seat_number: 0, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 7, seat_number: 1, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 0, seat_number: 0, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 10, seat_number: 1, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 10, seat_number: 2, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 10, seat_number: 3, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 7, seat_number: 2, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 7, seat_number: 3, food_order: Spinach, order_status: Ordered }
OrderItem { table_number: 7, seat_number: 4, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 8, seat_number: 0, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 8, seat_number: 1, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 8, seat_number: 2, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 9, seat_number: 0, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 9, seat_number: 1, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 9, seat_number: 2, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 0, seat_number: 1, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 0, seat_number: 2, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 0, seat_number: 3, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 1, seat_number: 0, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 1, seat_number: 1, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 1, seat_number: 2, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 1, seat_number: 3, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 1, seat_number: 4, food_order: Spinach, order_status: Ordered }
OrderItem { table_number: 1, seat_number: 5, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 2, seat_number: 0, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 2, seat_number: 1, food_order: Chicken, order_status: Ordered }
OrderItem { table_number: 2, seat_number: 2, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 2, seat_number: 3, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 3, seat_number: 0, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 3, seat_number: 1, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 3, seat_number: 2, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 3, seat_number: 3, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 3, seat_number: 4, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 3, seat_number: 5, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 4, seat_number: 0, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 4, seat_number: 1, food_order: Spinach, order_status: Ordered }
OrderItem { table_number: 4, seat_number: 2, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 4, seat_number: 3, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 15, seat_number: 0, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 15, seat_number: 1, food_order: Spinach, order_status: Ordered }
OrderItem { table_number: 15, seat_number: 2, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 15, seat_number: 3, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 15, seat_number: 4, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 15, seat_number: 5, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 16, seat_number: 0, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 16, seat_number: 1, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 17, seat_number: 0, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 17, seat_number: 1, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 17, seat_number: 2, food_order: Chicken, order_status: Ordered }
OrderItem { table_number: 17, seat_number: 3, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 17, seat_number: 4, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 17, seat_number: 5, food_order: Salad, order_status: Ordered }
OrderItem { table_number: 18, seat_number: 0, food_order: Chicken, order_status: Ordered }
OrderItem { table_number: 18, seat_number: 1, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 18, seat_number: 2, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 18, seat_number: 3, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 19, seat_number: 0, food_order: Noodles, order_status: Ordered }
OrderItem { table_number: 19, seat_number: 1, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 19, seat_number: 2, food_order: Spinach, order_status: Ordered }
OrderItem { table_number: 19, seat_number: 3, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 19, seat_number: 4, food_order: Chicken, order_status: Ordered }
OrderItem { table_number: 10, seat_number: 4, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 11, seat_number: 0, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 11, seat_number: 1, food_order: Yams, order_status: Ordered }
OrderItem { table_number: 11, seat_number: 2, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 12, seat_number: 0, food_order: Carrots, order_status: Ordered }
OrderItem { table_number: 12, seat_number: 1, food_order: Asparagus, order_status: Ordered }
OrderItem { table_number: 13, seat_number: 0, food_order: Coleslaw, order_status: Ordered }
OrderItem { table_number: 13, seat_number: 1, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 14, seat_number: 0, food_order: Fish, order_status: Ordered }
OrderItem { table_number: 14, seat_number: 1, food_order: Coleslaw, order_status: Ordered }
Closing time!
```

6) *Compile and run your code, and as the answer to this question please show the output that was produced.*
```rust
use rand::prelude::SliceRandom;
use rand::Rng;
use std::cell::RefCell;
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
    fn activate(&self, chef: Arc<Mutex<Chef>>);
    fn finish(&self) -> thread::Result<()>;
}

impl WaiterHandle for Arc<Mutex<Waiter>> {
    fn activate(&self, chef: Arc<Mutex<Chef>>) {
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
                                chef.lock().expect("Poisoned mutex").order(item);
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

    let arc_waiters: [Arc<Mutex<Waiter>>; NUM_WAITERS] =
        waiters.map(|waiter| Arc::new(Mutex::new(waiter)));

    let chef = Arc::new(Mutex::new(Chef::new()));
    chef.lock().expect("Poisoned mutex").activate();

    arc_waiters.iter().for_each(|aw| aw.activate(chef.clone()));

    arc_waiters.iter().for_each(|aw| {
        aw.finish().expect("Unable to join threads");
    });
    chef.lock().expect("Poisoned mutex").quit();
    chef.lock().expect("Poisoned mutex").finish();

    println!("Closing time!");
}
```
```shell
Ready for orders!
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
OrderItem { table_number: 0, seat_number: 0, food_order: Fish, order_status: Ready }
OrderItem { table_number: 0, seat_number: 1, food_order: Coleslaw, order_status: Ready }
OrderItem { table_number: 1, seat_number: 0, food_order: Salad, order_status: Ready }
OrderItem { table_number: 1, seat_number: 1, food_order: Asparagus, order_status: Ready }
OrderItem { table_number: 1, seat_number: 2, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 1, seat_number: 3, food_order: Fish, order_status: Ready }
OrderItem { table_number: 1, seat_number: 4, food_order: Asparagus, order_status: Ready }
OrderItem { table_number: 2, seat_number: 0, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 2, seat_number: 1, food_order: Spinach, order_status: Ready }
OrderItem { table_number: 2, seat_number: 2, food_order: Carrots, order_status: Ready }
OrderItem { table_number: 3, seat_number: 0, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 3, seat_number: 1, food_order: Spinach, order_status: Ready }
OrderItem { table_number: 3, seat_number: 2, food_order: Carrots, order_status: Ready }
OrderItem { table_number: 4, seat_number: 0, food_order: Coleslaw, order_status: Ready }
OrderItem { table_number: 4, seat_number: 1, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 4, seat_number: 2, food_order: Fish, order_status: Ready }
OrderItem { table_number: 4, seat_number: 3, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 15, seat_number: 0, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 15, seat_number: 1, food_order: Carrots, order_status: Ready }
OrderItem { table_number: 15, seat_number: 2, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 15, seat_number: 3, food_order: Asparagus, order_status: Ready }
OrderItem { table_number: 15, seat_number: 4, food_order: Carrots, order_status: Ready }
OrderItem { table_number: 15, seat_number: 5, food_order: Spinach, order_status: Ready }
OrderItem { table_number: 16, seat_number: 0, food_order: Salad, order_status: Ready }
OrderItem { table_number: 16, seat_number: 1, food_order: Asparagus, order_status: Ready }
OrderItem { table_number: 16, seat_number: 2, food_order: Carrots, order_status: Ready }
OrderItem { table_number: 16, seat_number: 3, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 16, seat_number: 4, food_order: Salad, order_status: Ready }
OrderItem { table_number: 16, seat_number: 5, food_order: Coleslaw, order_status: Ready }
OrderItem { table_number: 17, seat_number: 0, food_order: Noodles, order_status: Ready }
OrderItem { table_number: 17, seat_number: 1, food_order: Coleslaw, order_status: Ready }
OrderItem { table_number: 17, seat_number: 2, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 17, seat_number: 3, food_order: Yams, order_status: Ready }
OrderItem { table_number: 17, seat_number: 4, food_order: Salad, order_status: Ready }
OrderItem { table_number: 18, seat_number: 0, food_order: Coleslaw, order_status: Ready }
OrderItem { table_number: 18, seat_number: 1, food_order: Yams, order_status: Ready }
OrderItem { table_number: 18, seat_number: 2, food_order: Asparagus, order_status: Ready }
OrderItem { table_number: 18, seat_number: 3, food_order: Asparagus, order_status: Ready }
OrderItem { table_number: 18, seat_number: 4, food_order: Yams, order_status: Ready }
OrderItem { table_number: 19, seat_number: 0, food_order: Noodles, order_status: Ready }
OrderItem { table_number: 19, seat_number: 1, food_order: Salad, order_status: Ready }
OrderItem { table_number: 19, seat_number: 2, food_order: Salad, order_status: Ready }
OrderItem { table_number: 19, seat_number: 3, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 19, seat_number: 4, food_order: Noodles, order_status: Ready }
OrderItem { table_number: 5, seat_number: 0, food_order: Spinach, order_status: Ready }
OrderItem { table_number: 5, seat_number: 1, food_order: Coleslaw, order_status: Ready }
OrderItem { table_number: 5, seat_number: 2, food_order: Coleslaw, order_status: Ready }
OrderItem { table_number: 5, seat_number: 3, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 5, seat_number: 4, food_order: Coleslaw, order_status: Ready }
OrderItem { table_number: 5, seat_number: 5, food_order: Spinach, order_status: Ready }
OrderItem { table_number: 6, seat_number: 0, food_order: Yams, order_status: Ready }
OrderItem { table_number: 6, seat_number: 1, food_order: Spinach, order_status: Ready }
OrderItem { table_number: 6, seat_number: 2, food_order: Fish, order_status: Ready }
OrderItem { table_number: 6, seat_number: 3, food_order: Fish, order_status: Ready }
OrderItem { table_number: 7, seat_number: 0, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 7, seat_number: 1, food_order: Salad, order_status: Ready }
OrderItem { table_number: 8, seat_number: 0, food_order: Carrots, order_status: Ready }
OrderItem { table_number: 8, seat_number: 1, food_order: Coleslaw, order_status: Ready }
OrderItem { table_number: 8, seat_number: 2, food_order: Salad, order_status: Ready }
OrderItem { table_number: 8, seat_number: 3, food_order: Salad, order_status: Ready }
OrderItem { table_number: 8, seat_number: 4, food_order: Fish, order_status: Ready }
OrderItem { table_number: 9, seat_number: 0, food_order: Noodles, order_status: Ready }
OrderItem { table_number: 9, seat_number: 1, food_order: Yams, order_status: Ready }
OrderItem { table_number: 9, seat_number: 2, food_order: Carrots, order_status: Ready }
OrderItem { table_number: 9, seat_number: 3, food_order: Asparagus, order_status: Ready }
OrderItem { table_number: 9, seat_number: 4, food_order: Salad, order_status: Ready }
OrderItem { table_number: 10, seat_number: 0, food_order: Coleslaw, order_status: Ready }
OrderItem { table_number: 10, seat_number: 1, food_order: Fish, order_status: Ready }
OrderItem { table_number: 10, seat_number: 2, food_order: Asparagus, order_status: Ready }
OrderItem { table_number: 11, seat_number: 0, food_order: Yams, order_status: Ready }
OrderItem { table_number: 11, seat_number: 1, food_order: Carrots, order_status: Ready }
OrderItem { table_number: 11, seat_number: 2, food_order: Coleslaw, order_status: Ready }
OrderItem { table_number: 11, seat_number: 3, food_order: Salad, order_status: Ready }
OrderItem { table_number: 11, seat_number: 4, food_order: Asparagus, order_status: Ready }
OrderItem { table_number: 12, seat_number: 0, food_order: Carrots, order_status: Ready }
OrderItem { table_number: 12, seat_number: 1, food_order: Fish, order_status: Ready }
OrderItem { table_number: 12, seat_number: 2, food_order: Spinach, order_status: Ready }
OrderItem { table_number: 13, seat_number: 0, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 13, seat_number: 1, food_order: Noodles, order_status: Ready }
OrderItem { table_number: 13, seat_number: 2, food_order: Coleslaw, order_status: Ready }
OrderItem { table_number: 13, seat_number: 3, food_order: Yams, order_status: Ready }
OrderItem { table_number: 13, seat_number: 4, food_order: Fish, order_status: Ready }
OrderItem { table_number: 13, seat_number: 5, food_order: Salad, order_status: Ready }
OrderItem { table_number: 14, seat_number: 0, food_order: Spinach, order_status: Ready }
OrderItem { table_number: 14, seat_number: 1, food_order: Chicken, order_status: Ready }
OrderItem { table_number: 14, seat_number: 2, food_order: Spinach, order_status: Ready }
OrderItem { table_number: 14, seat_number: 3, food_order: Salad, order_status: Ready }
OrderItem { table_number: 14, seat_number: 4, food_order: Salad, order_status: Ready }
OrderItem { table_number: 14, seat_number: 5, food_order: Spinach, order_status: Ready }
Closing time!
```

7) *Compile and run your code, and as the answer to this question please show the output that was produced.*
```rust
use rand::prelude::SliceRandom;
use rand::Rng;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc;
use std::{
    sync::{Arc, Condvar, Mutex},
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

struct Waiter {
    handle: Option<thread::JoinHandle<()>>,
    tables: Vec<Table>,
    tx: mpsc::Sender<WorkItem>,
    rx: Option<mpsc::Receiver<WorkItem>>,
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
            let rx = guard.rx.take().expect("Waiter already activated");

            guard.handle = Some(thread::spawn(move || {
                let ordered = {
                    let guard = thread_waiter.lock().expect("Poisoned mutex");

                    println!("Welcome to AO's place!");

                    guard.tables.iter().for_each(|table| {
                        table.customers.iter().for_each(|customer| {
                            customer.order(&guard);
                        });
                    });

                    let mut count = 0;
                    guard.tables.iter().for_each(|table| {
                        table.customers.iter().for_each(|customer| {
                            customer
                                .order_items
                                .borrow_mut()
                                .drain(..)
                                .for_each(|item| {
                                    restaurant.chef.lock().expect("Poisoned mutex").order(item);
                                    count += 1;
                                });
                        });
                    });

                    count
                };

                restaurant.move_to_back(&thread_waiter);

                let mut served = 0;
                let mut got_quit = false;

                while !(got_quit && served == ordered) {
                    match rx.recv().expect("Channel closed") {
                        WorkItem::Quit => got_quit = true,

                        WorkItem::Process(mut item) => {
                            let owner = &restaurant.table_to_waiter[&item.table_number];

                            if Arc::ptr_eq(owner, &thread_waiter) {
                                restaurant.move_to_back(&thread_waiter);
                                item.order_status = Status::Served;
                                println!("{:?}", item);
                                served += 1;
                            } else {
                                owner
                                    .lock()
                                    .expect("Poisoned mutex")
                                    .pickup(WorkItem::Process(item));
                            }
                        }
                    }
                }
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
        let (tx, rx) = mpsc::channel();
        Self {
            handle: None,
            tables: Vec::new(),
            tx,
            rx: Some(rx),
        }
    }

    fn pickup(&self, work: WorkItem) {
        self.tx.send(work).expect("Waiter thread has stopped");
    }

    fn quit(&self) {
        self.tx
            .send(WorkItem::Quit)
            .expect("Waiter thread has stopped");
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

    fn activate(&mut self, restaurant: Arc<Restaurant>) {
        if self.handle.is_none() {
            let rx = self.rx.take().unwrap();

            self.handle = Some(thread::spawn(move || {
                println!("Ready for orders!");

                for item in rx {
                    match item {
                        WorkItem::Quit => break,
                        WorkItem::Process(mut order_item) => {
                            order_item.order_status = Status::Ready;

                            let leader = restaurant.leader();

                            leader
                                .lock()
                                .expect("Poisoned mutex")
                                .pickup(WorkItem::Process(order_item));
                        }
                    }
                }

                for w in &restaurant.waiters {
                    w.lock().expect("Poisoned mutex").quit();
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
    queue_changed: Condvar,
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
            queue_changed: Condvar::new(),
        }
    }

    // Get the Waiter at the front of the queue
    fn leader(&self) -> Arc<Mutex<Waiter>> {
        let q = self
            .queue_changed
            .wait_while(self.leader_queue.lock().expect("Poisoned mutex"), |q| {
                q.is_empty()
            })
            .expect("Poisoned mutex");

        Arc::clone(q.front().unwrap())
    }

    // Move some Waiter to back of the queue
    fn move_to_back(&self, w: &Arc<Mutex<Waiter>>) {
        let mut q = self.leader_queue.lock().expect("Poisoned mutex");

        // Remove from queue if it already is there
        if let Some(pos) = q.iter().position(|x| Arc::ptr_eq(x, w)) {
            q.remove(pos);
        }

        q.push_back(Arc::clone(w));

        self.queue_changed.notify_all();
    }

    // Waits for every waiter to be in the leader queue
    fn wait_all_in_queue(&self) {
        let _q = self
            .queue_changed
            .wait_while(self.leader_queue.lock().expect("Poisoned mutex"), |q| {
                q.len() < self.waiters.len()
            })
            .expect("Poisoned mutex");
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

    restaurant
        .chef
        .lock()
        .expect("Poisoned mutex")
        .activate(Arc::clone(&restaurant));

    for w in &restaurant.waiters {
        w.activate(Arc::clone(&restaurant));
    }

    // Wait until every waiter has delivered all its orders
    restaurant.wait_all_in_queue();

    // Chef finishes the remaining dishes, quits, then closes its thread
    restaurant.chef.lock().expect("Poisoned mutex").quit();
    restaurant.chef.lock().expect("Poisoned mutex").finish();

    // Waiters finish serving and exit
    for w in &restaurant.waiters {
        w.finish().expect("Unable to join waiter");
    }

    println!("Closing time!");
}
```
```shell
Ready for orders!
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
Welcome to AO's place!
OrderItem { table_number: 5, seat_number: 0, food_order: Chicken, order_status: Served }
OrderItem { table_number: 5, seat_number: 1, food_order: Chicken, order_status: Served }
OrderItem { table_number: 5, seat_number: 2, food_order: Carrots, order_status: Served }
OrderItem { table_number: 5, seat_number: 3, food_order: Noodles, order_status: Served }
OrderItem { table_number: 5, seat_number: 4, food_order: Fish, order_status: Served }
OrderItem { table_number: 5, seat_number: 5, food_order: Fish, order_status: Served }
OrderItem { table_number: 6, seat_number: 0, food_order: Salad, order_status: Served }
OrderItem { table_number: 6, seat_number: 1, food_order: Chicken, order_status: Served }
OrderItem { table_number: 6, seat_number: 2, food_order: Carrots, order_status: Served }
OrderItem { table_number: 6, seat_number: 3, food_order: Coleslaw, order_status: Served }
OrderItem { table_number: 6, seat_number: 4, food_order: Spinach, order_status: Served }
OrderItem { table_number: 6, seat_number: 5, food_order: Noodles, order_status: Served }
OrderItem { table_number: 7, seat_number: 0, food_order: Asparagus, order_status: Served }
OrderItem { table_number: 7, seat_number: 1, food_order: Asparagus, order_status: Served }
OrderItem { table_number: 7, seat_number: 2, food_order: Fish, order_status: Served }
OrderItem { table_number: 7, seat_number: 3, food_order: Carrots, order_status: Served }
OrderItem { table_number: 8, seat_number: 0, food_order: Coleslaw, order_status: Served }
OrderItem { table_number: 8, seat_number: 1, food_order: Noodles, order_status: Served }
OrderItem { table_number: 8, seat_number: 2, food_order: Carrots, order_status: Served }
OrderItem { table_number: 8, seat_number: 3, food_order: Chicken, order_status: Served }
OrderItem { table_number: 8, seat_number: 4, food_order: Fish, order_status: Served }
OrderItem { table_number: 9, seat_number: 0, food_order: Fish, order_status: Served }
OrderItem { table_number: 9, seat_number: 1, food_order: Fish, order_status: Served }
OrderItem { table_number: 9, seat_number: 2, food_order: Carrots, order_status: Served }
OrderItem { table_number: 9, seat_number: 3, food_order: Yams, order_status: Served }
OrderItem { table_number: 10, seat_number: 0, food_order: Coleslaw, order_status: Served }
OrderItem { table_number: 10, seat_number: 1, food_order: Asparagus, order_status: Served }
OrderItem { table_number: 10, seat_number: 2, food_order: Carrots, order_status: Served }
OrderItem { table_number: 10, seat_number: 3, food_order: Spinach, order_status: Served }
OrderItem { table_number: 10, seat_number: 4, food_order: Chicken, order_status: Served }
OrderItem { table_number: 10, seat_number: 5, food_order: Fish, order_status: Served }
OrderItem { table_number: 11, seat_number: 0, food_order: Chicken, order_status: Served }
OrderItem { table_number: 11, seat_number: 1, food_order: Spinach, order_status: Served }
OrderItem { table_number: 11, seat_number: 2, food_order: Yams, order_status: Served }
OrderItem { table_number: 11, seat_number: 3, food_order: Spinach, order_status: Served }
OrderItem { table_number: 12, seat_number: 0, food_order: Noodles, order_status: Served }
OrderItem { table_number: 12, seat_number: 1, food_order: Chicken, order_status: Served }
OrderItem { table_number: 12, seat_number: 2, food_order: Asparagus, order_status: Served }
OrderItem { table_number: 13, seat_number: 0, food_order: Noodles, order_status: Served }
OrderItem { table_number: 13, seat_number: 1, food_order: Noodles, order_status: Served }
OrderItem { table_number: 13, seat_number: 2, food_order: Coleslaw, order_status: Served }
OrderItem { table_number: 13, seat_number: 3, food_order: Asparagus, order_status: Served }
OrderItem { table_number: 13, seat_number: 4, food_order: Noodles, order_status: Served }
OrderItem { table_number: 13, seat_number: 5, food_order: Yams, order_status: Served }
OrderItem { table_number: 14, seat_number: 0, food_order: Asparagus, order_status: Served }
OrderItem { table_number: 14, seat_number: 1, food_order: Salad, order_status: Served }
OrderItem { table_number: 14, seat_number: 2, food_order: Coleslaw, order_status: Served }
OrderItem { table_number: 14, seat_number: 3, food_order: Carrots, order_status: Served }
OrderItem { table_number: 0, seat_number: 0, food_order: Fish, order_status: Served }
OrderItem { table_number: 0, seat_number: 1, food_order: Noodles, order_status: Served }
OrderItem { table_number: 0, seat_number: 2, food_order: Carrots, order_status: Served }
OrderItem { table_number: 0, seat_number: 3, food_order: Coleslaw, order_status: Served }
OrderItem { table_number: 0, seat_number: 4, food_order: Carrots, order_status: Served }
OrderItem { table_number: 0, seat_number: 5, food_order: Asparagus, order_status: Served }
OrderItem { table_number: 1, seat_number: 0, food_order: Noodles, order_status: Served }
OrderItem { table_number: 1, seat_number: 1, food_order: Yams, order_status: Served }
OrderItem { table_number: 2, seat_number: 0, food_order: Spinach, order_status: Served }
OrderItem { table_number: 2, seat_number: 1, food_order: Salad, order_status: Served }
OrderItem { table_number: 3, seat_number: 0, food_order: Asparagus, order_status: Served }
OrderItem { table_number: 3, seat_number: 1, food_order: Yams, order_status: Served }
OrderItem { table_number: 3, seat_number: 2, food_order: Yams, order_status: Served }
OrderItem { table_number: 3, seat_number: 3, food_order: Coleslaw, order_status: Served }
OrderItem { table_number: 3, seat_number: 4, food_order: Yams, order_status: Served }
OrderItem { table_number: 3, seat_number: 5, food_order: Coleslaw, order_status: Served }
OrderItem { table_number: 4, seat_number: 0, food_order: Yams, order_status: Served }
OrderItem { table_number: 4, seat_number: 1, food_order: Spinach, order_status: Served }
OrderItem { table_number: 4, seat_number: 2, food_order: Noodles, order_status: Served }
OrderItem { table_number: 4, seat_number: 3, food_order: Spinach, order_status: Served }
OrderItem { table_number: 4, seat_number: 4, food_order: Fish, order_status: Served }
OrderItem { table_number: 15, seat_number: 0, food_order: Fish, order_status: Served }
OrderItem { table_number: 15, seat_number: 1, food_order: Noodles, order_status: Served }
OrderItem { table_number: 15, seat_number: 2, food_order: Yams, order_status: Served }
OrderItem { table_number: 16, seat_number: 0, food_order: Chicken, order_status: Served }
OrderItem { table_number: 16, seat_number: 1, food_order: Salad, order_status: Served }
OrderItem { table_number: 16, seat_number: 2, food_order: Carrots, order_status: Served }
OrderItem { table_number: 16, seat_number: 3, food_order: Yams, order_status: Served }
OrderItem { table_number: 17, seat_number: 0, food_order: Carrots, order_status: Served }
OrderItem { table_number: 17, seat_number: 1, food_order: Carrots, order_status: Served }
OrderItem { table_number: 18, seat_number: 0, food_order: Spinach, order_status: Served }
OrderItem { table_number: 18, seat_number: 1, food_order: Yams, order_status: Served }
OrderItem { table_number: 18, seat_number: 2, food_order: Coleslaw, order_status: Served }
OrderItem { table_number: 18, seat_number: 3, food_order: Noodles, order_status: Served }
OrderItem { table_number: 19, seat_number: 0, food_order: Chicken, order_status: Served }
OrderItem { table_number: 19, seat_number: 1, food_order: Spinach, order_status: Served }
Closing time!
```

AI Log: https://claude.ai/share/703f1803-2e7c-4660-a8b3-511a222ac237
