fn main() {
    let mut x: u8 = 1;

    #[cfg(old_exercise)]
    loop {
        println!("x: {}", x);
        x *= 2;
    }

    #[cfg(old_exercise)]
    while let Some(next) = x.checked_mul(2) {
        println!("x: {}", x); 
        x = next;
    }

    #[cfg(old_exercise)]
    {
        let mut doublings: usize = 0;
        while !x.overflowing_mul(2).1 {
            println!("x = {} -- # of doublings = {}", x, doublings);
            x *= 2;
            doublings += 1;
        }
        println!("x = {} -- # of doublings = {}", x, doublings);
    }

    for _ in 0..8 {
        x = x.saturating_mul(2);
        println!("x = {}", x);
    }
}
