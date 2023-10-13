/* 10.Create a function that takes a sentence as input and converts it to "Title Case"(capitalize the first letter of each word) while
ignoring common articles and prepositions like "the," "in," "of," etc. */
pub fn capitalize_1st_letter(sentence: &str) -> String {
    let ignored_words = vec!["the", "in", "of", "and", "a", "an"];
    let mut new_sentence = String::new();
    let mut capitalize_next = true;

    for word in sentence.split_whitespace() {
        let lowercase_word = word.to_ascii_lowercase();
        if !ignored_words.contains(&lowercase_word.as_str()) {
            if capitalize_next {
                new_sentence.push_str(&lowercase_word[..1].to_ascii_uppercase());
                new_sentence.push_str(&lowercase_word[1..]);
                capitalize_next = false;
            } else {
                new_sentence.push_str(&lowercase_word);
            }
        } else {
            new_sentence.push_str(&lowercase_word);
        }
        new_sentence.push(' ');
        capitalize_next = true; // Set this to true after each word
    }

    new_sentence.pop(); // Remove the trailing space
    new_sentence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_1st_letter() {
        //assert_eq!(capitalize_1st_letter("hello world"), "Hello World");
        assert_eq!(
            capitalize_1st_letter("i want the french fries"),
            "I Want the French Fries"
        );
    }
}
