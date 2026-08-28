mod my {
    #[derive(Debug)]
    pub enum Err {
        DivByZero = 1
    }
}

fn main() -> Result<(), my::Err> {
    println!("Hello, world!");

    let x = 45;
    let y = 23;
    let z = x - y + 2;

    let numerator = x;
    let denominator = x - y - z;

    match denominator {
        0 => Err(my::Err::DivByZero),
        _ => {
            println!("Result: {}", numerator / denominator);
            Ok(())
        }
    }
}
