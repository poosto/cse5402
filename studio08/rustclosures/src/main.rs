struct Foo<T>(T, fn(T) -> T, fn(T) -> T);

fn main() {
    let x: f64 = 0.0;

    let mut s = Foo(x, f64::cos, f64::sin);

    let mut c = || s.0 = s.2(s.1(s.0));
    c();

    println!("x = {}", s.0);

    let mut word: String = "Hello".to_string();
    println!("Before: {}", word);

    let mut extend_word = || word += ", world!";
    extend_word();
    println!("After: {}", word);
}
