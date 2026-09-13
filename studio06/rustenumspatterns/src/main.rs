fn main() {
    #[cfg(oldexercise)]
    {
        let x = ("hello", 45u32);

        match x {
            (name, number) => println!("Matched: {} {}", name, number)
        };

        // or...

        let (name, number) = x;
        println!("Matched: {} {}", name, number);
    }

    use std::str::FromStr;

    let x = "Hello";

    #[cfg(oldexercise)]
    match u8::from_str(x) {
        Ok(number) => println!("Got value: {}", number),
        Err(_) => println!("Could not convert string literal to u8")
    };

    if let Ok(number) = u8::from_str(x) {
        println!("Got value: {}", number);
    } else {
        println!("Could not convert string literal to u8");
    }
}
