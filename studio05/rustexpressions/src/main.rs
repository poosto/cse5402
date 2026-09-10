fn main() {
    #[cfg(oldexercise)]
    println!("Compute: {}", 3 + 4 * 8);

    #[cfg(oldexercise)]
    {
        let x = 35;
        let y = 63;

        let result = if x > y {
            x;
        } else {
            y
        };

        println!("Result: {}", result);
    }

    #[cfg(oldexercise)]
    {
        let tup = (35, 63);

        // let result = match tup {
        //     (x, y) => {
        //         if x > y {
        //             x
        //         } else {
        //             y
        //         }
        //     }
        // };

        // Simplified match destructuring
        // (match is just a let in disguise, but let requires an irrefutable pattern)
        let (x, y) = tup;
        let result = if x > y {
            x
        } else {
            y
        };

        println!("Result: {}", result);
    }

    let mut vec = Vec::<usize>::new();

    for i in 0..10 {
        vec.push(i);
    }
    
    println!("Vec: {:?}", vec); 

    for j in &vec {
        println!("Value: {}", j);
    }

    println!("Vec again: {:?}", vec);
}
