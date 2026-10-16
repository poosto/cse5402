### Studio 10

1) *As the answer to the first exercise, list the names of the people who worked together on this studio.* \
Preston Meek

2) *Compile and run your program, and as the answer to this exercise please show the code you wrote and the output the
program produced.*
```rust
fn main() {
    let text = "Thanks, Rosencrantz and gentle Guildenstern.".to_string();
    let vec: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();

    println!("Vec: {:?}", vec);
}
```
```shell
Vec: ["Thanks,", "Rosencrantz", "and", "gentle", "Guildenstern."]
```

3) *Compile and run your program, and as the answer to this exercise please (1) show the output that was produced and
(2) explain how and why the output differed for the BTreeSet vs. the vector.*
```shell
Vec: ["Thanks,", "Rosencrantz", "and", "gentle", "Guildenstern."]
BTree: {"Guildenstern.", "Rosencrantz", "Thanks,", "and", "gentle"}
```
The output differed because the BTree sorted the individual words in its container. It first sorted by uppercase, then
by lowercase, since this is the order of ASCII codes.

4) *Compile and run your program, and as the answer to this exercise please (1) show the output that was produced and
(2) explain how and why the output differed for the HashSet vs. the BTreeSet or the vector.* 
```shell
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustcollections`
Vec: ["Thanks,", "Rosencrantz", "and", "gentle", "Guildenstern."]
BTree: {"Guildenstern.", "Rosencrantz", "Thanks,", "and", "gentle"}
Hash: {"and", "Thanks,", "gentle", "Guildenstern.", "Rosencrantz"}

CSE 5402/studio10/rustcollections on  main [?] is 📦 v0.1.0 via 🦀 v1.87.0
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rustcollections`
Vec: ["Thanks,", "Rosencrantz", "and", "gentle", "Guildenstern."]
BTree: {"Guildenstern.", "Rosencrantz", "Thanks,", "and", "gentle"}
Hash: {"Thanks,", "and", "Guildenstern.", "Rosencrantz", "gentle"}
```
The output for the HashSet is effectively random, since it provides no ordering guarantees.

5) *Compile and run your program, and as the answer to this exercise please (1) show the output that was produced and
(2) explain why the HashSet and BTreeSet collections ended up containing fewer items than the vector.*
```shell
Vec: ["She", "sells", "sea", "shells", "by", "the", "sea", "shore."]
BTree: {"She", "by", "sea", "sells", "shells", "shore.", "the"}
Hash: {"sea", "the", "by", "shells", "She", "shore.", "sells"}
```
The Vec contains one less element because 'sea' repeats twice, and BTreeSet and HashSet guarantee that they only hold
unique elements.

6) *Compile and run your program, and as the answer to this exercise please (1) show the output that was produced and
(2) explain which other collection's contents are most similar to those of that BTreeMap, and why.* 
```shell
Vec: ["She", "sells", "sea", "shells", "by", "the", "sea", "shore."]
BTree: {"She", "by", "sea", "sells", "shells", "shore.", "the"}
Hash: {"sells", "She", "by", "sea", "the", "shells", "shore."}
BMap: {0: "She", 1: "sells", 2: "sea", 3: "shells", 4: "by", 5: "the", 6: "sea", 7: "shore."}
```
The container with the most similar contents is Vec, since internally a Vec is essentially (idx, value) pairings. This
is exactly what the BTreeMap allows, since its keys are guaranteed to be sorted. Additionally, it can have duplicate
words (values) as long as it doesn't have duplicate indices (keys).

7) *Compile and run your program, and as the answer to this exercise please (1) show the output that was produced and
(2) explain which other collection's contents are most similar to those of the second BTreeMap, and why.* 
```shell
Vec: ["She", "sells", "sea", "shells", "by", "the", "sea", "shore."]
BTree: {"She", "by", "sea", "sells", "shells", "shore.", "the"}
Hash: {"sea", "She", "sells", "shells", "by", "the", "shore."}
BMap: {0: "She", 1: "sells", 2: "sea", 3: "shells", 4: "by", 5: "the", 6: "sea", 7: "shore."}
BMapRev: {"She": 0, "by": 4, "sea": 6, "sells": 1, "shells": 3, "shore.": 7, "the": 5}
```
This container is most similar to the BTreeSet, since its keys match that container in terms of ordering and value. This
new BTreeMap's values are somewhat meaningless; they represent the original indices into the Vec, but it is missing one
of the 'sea' values since BTreeMaps can't have repeating keys.

8) *As the answer to this exercise, please (1) show the error that occurred, and (2) explain briefly why that cloning
was needed.* 
```shell
error[E0277]: a value of type `BTreeMap<usize, String>` cannot be built from an iterator over elements of type `(usize, &String)`
    --> src/main.rs:8:64
     |
8    | ...vec.iter().enumerate().collect();
     |                           ^^^^^^^ value of type `BTreeMap<usize, String>` cannot be built from `std::iter::Iterator<Item=(usize, &String)>`
     |
     = help: the trait `FromIterator<(_, &String)>` is not implemented for `BTreeMap<usize, String>`
             but trait `FromIterator<(_, String)>` is implemented for it
     = help: for that trait implementation, expected `String`, found `&String`
note: the method call chain might not have had the expected associated types
    --> src/main.rs:8:52
     |
4    | ... = "She sells sea shells by the sea shore.".to_string();
     |       ---------------------------------------- this expression has type `&str`
5    | ... Vec<String> = text.split_whitespace().map(|s| s.to_string()).co...
     |                        ------------------ ---------------------- `Iterator::Item` changed to `String` here
     |                        |
     |                        `Iterator::Item` is `&str` here
...
8    | ...: BTreeMap<usize, String> = vec.iter().enumerate().collect();
     |                                    ------ ^^^^^^^^^^^ `Iterator::Item` changed to `(usize, &String)` here
     |                                    |
     |                                    `Iterator::Item` is `&String` here
note: required by a bound in `collect`
    --> /Users/prestonmeek/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:1972:19
     |
1972 |     fn collect<B: FromIterator<Self::Item>>(self) -> B
     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator::collect`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustcollections` (bin "rustcollections") due to 1 previous error
```
The cloning is necessary here because a container owns its values. When creating the containers from the string slices,
the to_string method created owned copies of the static strings. However,  when trying to initialize a container from
another container, those owned values do not implement Copy and therefore need to be explicitly Cloned. The original
container could also be moved, but this is not the behavior we want.

9) *Compile and run your program, and as the answer to this exercise please show the code you wrote and the output the
program produced.*
```shell
upper_count: 1
lower_count: 9
str len - upper_count - lower_count = 3
```

10) *Compile and run your program, and as the answer to this exercise please show the code you added and the output the
program produced.*
```rust
fn main() {
    let hello = "Hello, world!";

    let upper_count = hello.chars().filter(char::is_ascii_uppercase).count();
    let lower_count = hello.chars().filter(char::is_ascii_lowercase).count();
    let diff = hello.len() - upper_count - lower_count;

    let rev: String = hello.chars().rev().collect();

    println!("upper_count: {}", upper_count);
    println!("lower_count: {}", lower_count);
    println!("str len - upper_count - lower_count = {}", diff);

    println!("{}\n{}", hello, rev);
}
```
```shell
upper_count: 1
lower_count: 9
str len - upper_count - lower_count = 3
Hello, world!
!dlrow ,olleH
```

11) *Compile and run your program, and as the answer to this exercise please show the output that was produced.*
```shell
upper_count: 1
lower_count: 9
str len - upper_count - lower_count = 3
Hello, world!
!dlrow ,olleH
kayak? true
administration? false
```

12) *Compile and run your program, and as the answer to this exercise please show the output that was produced.*
```shell
upper_count: 1
lower_count: 9
str len - upper_count - lower_count = 3
Hello, world!
!dlrow ,olleH
Is kayak a palindrome? yes!
Is administration a palindrome? no!
Is a7 6b b67a a palindrome? yes!
```

13) *Compile and run your program, and as the answer to this exercise please show the output that was produced.*
```shell
Is kayak a palindrome? yes!
Is administration a palindrome? no!
Is a7 6b b67a a palindrome? yes!
Is Madam, I'm Adam. a palindrome? yes!
```

14) *Compile and run your program, and as the answer to this exercise please show the output that was produced.*
```shell
upper_count: 1
lower_count: 9
str len - upper_count - lower_count = 3
Hello, world!
!dlrow ,olleH
Is kayak a palindrome? yes!
Is administration a palindrome? no!
Is a7 6b b67a a palindrome? yes!
Is Madam, I'm Adam. a palindrome? yes!
Is Eh! Ça va, la vache? a palindrome? yes!
```


15) *Compile and run your program, and as the answer to this exercise please show (1) the output that was produced, and
(2) your program's entire code including the functions it uses.*
```rust
// Function generated with Claude
fn latin1_to_ascii(c: char) -> char {
    match c {
        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' | 'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => 'a',
        'Ç' | 'ç' => 'c',
        'Ð' | 'ð' => 'd',
        'È'..='Ë' => 'e', // line added by me!
        'Ì' | 'Í' | 'Î' | 'Ï' | 'ì' | 'í' | 'î' | 'ï' => 'i',
        'Ñ' | 'ñ' => 'n',
        'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' | 'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' => 'o',
        'Ù' | 'Ú' | 'Û' | 'Ü' | 'ù' | 'ú' | 'û' | 'ü' => 'u',
        'Ý' | 'ý' | 'ÿ' => 'y',
        _ => c,
    }
}

fn is_palindrome(s: &str) -> bool {
    let s = s
        .chars()
        .map(latin1_to_ascii)
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase());

    let forward = s.clone().collect::<String>();
    let reverse = s.rev().collect::<String>();

    forward == reverse
}

fn print_palindrome(s: &str) {
    println!(
        "Is {} a palindrome? {}",
        s,
        if is_palindrome(s) { "yes!" } else { "no!" }
    );
}

fn main() {
    let hello = "Hello, world!";

    let upper_count = hello.chars().filter(char::is_ascii_uppercase).count();
    let lower_count = hello.chars().filter(char::is_ascii_lowercase).count();
    let diff = hello.len() - upper_count - lower_count;

    let rev: String = hello.chars().rev().collect();

    println!("upper_count: {}", upper_count);
    println!("lower_count: {}", lower_count);
    println!("str len - upper_count - lower_count = {}", diff);

    println!("{}\n{}", hello, rev);

    print_palindrome("kayak");
    print_palindrome("administration");
    print_palindrome("a7 6b b67a");
    print_palindrome("Madam, I'm Adam.");
    print_palindrome("Eh! Ça va, la vache?");
    print_palindrome("Ésope reste ici et se repose.");
}
```
```shell
upper_count: 1
lower_count: 9
str len - upper_count - lower_count = 3
Hello, world!
!dlrow ,olleH
Is kayak a palindrome? yes!
Is administration a palindrome? no!
Is a7 6b b67a a palindrome? yes!
Is Madam, I'm Adam. a palindrome? yes!
Is Eh! Ça va, la vache? a palindrome? yes!
Is Ésope reste ici et se repose. a palindrome? yes!
```
