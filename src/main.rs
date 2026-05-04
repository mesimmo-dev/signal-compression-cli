use std::env;

fn main() {
    let input: String = env::args().skip(1).collect::<Vec<String>>().join(" ");

    if input.trim().is_empty() {
        println!("No input provided.");
        println!("Usage: cargo run -- \"your text here\"");
        return;
    }

    let word_count = input.split_whitespace().count();
    let char_count = input.chars().count();

    let preview: String = input
        .split_whitespace()
        .take(12)
        .collect::<Vec<&str>>()
        .join(" ");

    println!("Signal Compression Report");
    println!("-------------------------");
    println!("Words: {}", word_count);
    println!("Characters: {}", char_count);
    println!("Compressed Preview: {}...", preview);
}
