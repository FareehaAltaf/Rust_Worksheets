use std::collections::HashMap;
use std::fs;

fn main() {
    let file_contents = match fs::read_to_string("file.txt") {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Failed to read the file.");
            return;
        }
    };

    let mut word_count: HashMap<String, u32> = HashMap::new();

    for word in file_contents.split_whitespace() {
        // Remove punctuation and convert to lowercase
        let cleaned_word = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>()
            .to_lowercase();

        // Count word occurrences
        *word_count.entry(cleaned_word).or_insert(0) += 1;
    }

    for (word, count) in &word_count {
        println!("Word: {}, Count: {}", word, count);
    }
}
