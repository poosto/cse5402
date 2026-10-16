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
