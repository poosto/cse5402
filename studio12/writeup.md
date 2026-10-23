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
