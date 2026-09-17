use std::fmt::Display;

#[derive(Debug, Default)]
// usually, you shouldn't have trait bounds on the struct itself, only impl blocks
// but, any trait bounds on Drop must exist on the struct itself, so...
struct Foo<T: Display> {
    one: T,
    two: T
}

impl<T: Display> Foo<T> {
    fn new<U: Into<T>>(one: U, two: U) -> Self {
        Foo {
            one: one.into(), two: two.into()
        }
    }
}

// different from std::mem::drop,
// since that takes by ownership (intuitively)
impl<T: Display> Drop for Foo<T> {
    fn drop(&mut self) {
        println!("Dropping struct {} {}", self.one, self.two);
    }
}

fn main() {
    let foo_one = Foo::<String>::new("hi", "there");
    let foo_two = Foo::<usize>::new(45_usize, 33_usize);

    let foo_one_prime = Foo::<String>::new("hi", Default::default());
    let foo_two_prime = Foo::<usize>::new(45_usize, Default::default());

    let foo_three = Foo::<String>{ one: "hi".to_string(), ..Foo::<String>::default() };

    println!("{:?} {:?}\n{:?} {:?}", foo_one, foo_two, foo_one_prime, foo_two_prime);
    println!("{:?}", foo_three);
}
