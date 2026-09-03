fn main() {
    #[cfg(oldexercise)]
    {
        let a: u8 = 4;
        let b: u8 = 6;
        let c: u8 = b;

        println!("Values: {} {} {}", a, b, c);
    }

    let a: String = "hello".to_string();
    let mut b: String = "there".to_string();
    let mut c: String = b;
    b = "world".to_string();

    println!("Values: {} {} {}", a, b, c);

    b = c;
    c = "buddy".to_string();
    println!("Values: {} {} {}", a, b, c);

    let mut vec = Vec::<String>::new();
    vec.push(a);
    vec.push(b);
    vec.push(c);
    println!("Vec values: {:?}", vec);

    let d: String = vec.pop().expect("should have a value here");
    println!("Vec + str: {:?} {}", vec, d);
}
