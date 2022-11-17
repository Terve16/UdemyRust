
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &s[..i];
        }
    }

    return &s;
}

fn main() {
    let mut hello = String::from("Hello world! 😁 🎈");
    // string literal zu string objekt "casten" mit from funktion
    // Variablen sind standardmässig nicht veränderbar, Konstanten
    // mutable; immutable

    println!("{}", hello);
    hello.push('w');
    println!("{}", hello);

    // println!("{}", hello[0]);

    for b in hello.bytes() {
        print!("{} ", b);
    }

    println!("");

    for c in hello.chars() {
        print!("{}", c);
    }

    let word = first_word(&hello);
    println!("{}", word);
}
