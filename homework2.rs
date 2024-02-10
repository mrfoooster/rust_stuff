use std::io::{self, Write};

struct WordCounter {
    text: String,
}

impl WordCounter {
    fn new(text: &str) -> WordCounter {
        WordCounter { text: text.to_string() }
    }

    fn count_words(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

fn main() {
    println!("Enter a text:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();

    if input.is_empty() {
        println!("Error: Please enter some text.");
        return;
    }

    let word_counter = WordCounter::new(&input);
    let word_count = word_counter.count_words();

    println!("Word count: {}", word_count);
}
