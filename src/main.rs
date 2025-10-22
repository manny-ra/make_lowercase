use std::io::{self, Read};

fn main() {
    println!("Paste your text (Ctrl+D to finish):");

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("failed to read line");
    input = input.trim_end().to_string();

    let mut result = String::with_capacity(input.capacity());

    let mut iter = input.chars().peekable();

    while let Some(c) = iter.next() {
        match (c, iter.peek()) {
            // I followed by known self-reference indicators
            ('I', Some(' ' | '\'' | '’')) => result.push(c),

            // I followed by punctuation, still likely self-reference
            ('I', Some('.' | ',' | '!' | '?' | ':' | ';' | '-' | '–' | '/')) => result.push(c),

            // I at the end of input without punctuation
            ('I', None) => result.push(c),

            _ => result.extend(c.to_lowercase()),
        };
    }

    println!("{}", result);
}
