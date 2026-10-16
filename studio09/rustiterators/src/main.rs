fn main() {
    #[cfg(oldexercise)]
    {
        let mut vec = Vec::<bool>::new();
        let mut x: u32 = 0;
        for i in 0_u32..=99 {
            let keep = i == 2 || (i > 2 && i % 2 != 0);
            vec.push(keep);

            if keep {
                let sep = if x % 10 == 9 { "\n" } else { " " };
                let s = format!("{}{}", i, sep);
                x += 1;

                print!("{}", s);
            }
        }
    }

    let mut vec: Vec<bool> = (0_u32..=99)
        .map(|i| i == 2 || (i > 2 && i % 2 != 0))
        .collect();

    // this algorithm is inherently imperative,
    // so can't really make it any more functional.
    (3usize..=99).for_each(|i| {
        if vec[i] {
            for j in (i * i..=99).step_by(i) {
                vec[j] = false;
            }
        }
    });

    // but this is a cool declarative approach!
    vec = (0_u32..=99)
        // map every number to a boolean
        // returns true if n >= 2
        // AND (2..) (RangeFrom, no end) until d*d <= n aka up to d <= sqrt(n)
        // and if all of those do not divide into n.
        // ex, for 23, it takes (2,3,4)
        // then does 23%2, 23%3, and 23%4. all are != 0, so it returns true!
        .map(|n| n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| n % d != 0))
        .collect();

    let nums: Vec<String> = vec
        .iter()
        .enumerate()
        .filter(|&(_, &keep)| keep)
        .map(|(i, _)| i.to_string())
        .collect();

    let output = nums
        .chunks(10)
        .map(|line| line.join(" "))
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", output);
}
