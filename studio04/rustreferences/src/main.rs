fn main() {
    #[cfg(oldexercise)]
    {
        let x = 45;
        let y = &x;
        let z = &y;

        let aa = *y == **z;

        println!("Values: {} {} {}", x, y, z);
        println!("Comparison: {}", aa);
    }

    #[cfg(oldexercise)]
    {
        let mut x = 45;
        let y = &mut x;
        println!("Value: {}", y);

        *y = 33;
        println!("New value: {}", y);
    }

    let mut s = "hello world".to_string();
    {
        let mref = &mut s;
        println!("aliases: {}", mref);
        *mref = "hey there".to_string();
        println!("now aliases: {}", mref);
    }
    println!("str: {}", s);
}

