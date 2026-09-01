#[derive(Debug)]
struct Point<T>(T, T);

impl<T> Point<T> {
    fn new(first: T, second: T) -> Self {
        Point(first, second)
    }

    fn set(&mut self, first: T, second: T) {
        self.0 = first;
        self.1 = second;
    }
}

fn main() {
    let mut p = Point::<f64>::new(1.3, 2.3);
    println!("p is {:?}", p);

    p.set(3.3, 4.6);
    println!("p is now {:?}", p);
}
