### Studio 09

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Compile and run your program, and as the answer to this exercise please show your code and the output the program
produced.*
```rust
fn main() {
    for i in 0_u32..=99 {
        // \n every 10th character
        let sep = if i % 10 == 9 { "\n" } else { " " };
        let s = format!("{}{}", i, sep);

        print!("{}", s);
    }
}
```
```shell
0 1 2 3 4 5 6 7 8 9
10 11 12 13 14 15 16 17 18 19
20 21 22 23 24 25 26 27 28 29
30 31 32 33 34 35 36 37 38 39
40 41 42 43 44 45 46 47 48 49
50 51 52 53 54 55 56 57 58 59
60 61 62 63 64 65 66 67 68 69
70 71 72 73 74 75 76 77 78 79
80 81 82 83 84 85 86 87 88 89
90 91 92 93 94 95 96 97 98 99
```

3) *Compile and run your program, and as the answer to this exercise please show the output that was produced.*
```rust
fn main() {
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
```
```shell
2 3 5 7 9 11 13 15 17 19
21 23 25 27 29 31 33 35 37 39
41 43 45 47 49 51 53 55 57 59
61 63 65 67 69 71 73 75 77 79
81 83 85 87 89 91 93 95 97 99
```

4) *As the answer to this exercise please show the code that initializes the vector.*
```rust
    let vec: vec<bool> = (0_u32..=99)
        .map(|i| i == 2 || (i > 2 && i % 2 != 0))
        .collect();

    let nums: vec<string> = vec
        .iter()
        .enumerate()
        .filter(|&(_, &keep)| keep)
        .map(|(i, _)| i.to_string())
        .collect();

    let output = nums
        .chunks(10)
        .map(|line| line.join(" "))
        .collect::<vec<_>>()
        .join("\n");

    println!("{}", output);
```
```shell
2 3 5 7 9 11 13 15 17 19
21 23 25 27 29 31 33 35 37 39
41 43 45 47 49 51 53 55 57 59
61 63 65 67 69 71 73 75 77 79
81 83 85 87 89 91 93 95 97 99
```

5) *Compile and run your program, and as the answer to this exercise please show the output the program produced.*
```rust
    let mut vec: Vec<bool> = (0_u32..=99)
        .map(|i| i == 2 || (i > 2 && i % 2 != 0))
        .collect();

    for i in 0usize..=99 {
        if vec[i] {
            for j in (i..=99).step_by(i).skip(1) {
                vec[j] = false;
            }
        }
    }

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
```
```shell
2 3 5 7 11 13 17 19 23 29
31 37 41 43 47 53 59 61 67 71
73 79 83 89 97
```

6) *As the answer to this exercise please show the code that uses the `for_each` method with the closure, to mark
multiples false in the vector.*
```rust
    let mut vec: Vec<bool> = (0_u32..=99)
        .map(|i| i == 2 || (i > 2 && i % 2 != 0))
        .collect();

    (3usize..=99).for_each(|i| {
        if vec[i] {
            for j in (i..=99).step_by(i).skip(1) {
                vec[j] = false;
            }
        }
    });

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
```

7) *As the answer to this exercise please show the updated code with that optimization.*
```rust
    (3usize..=99).for_each(|i| {
        if vec[i] {
            for j in (i * i..=99).step_by(i) {
                vec[j] = false;
            }
        }
    });
```
```shell
2 3 5 7 11 13 17 19 23 29
31 37 41 43 47 53 59 61 67 71
73 79 83 89 97
```
