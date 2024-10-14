use std::collections::HashMap;

/// Counts the frequency of each character in a given string
/// # Arguments
/// * `text`: The string to count the frequency of
/// # Returns
/// A string formatted like a table, with each character and its count
/// separated by a newline
pub fn count_frequency(text: String) -> String {
    let mut freq = HashMap::new();

    for letter in text.chars() {
        let count = freq.get(&letter).unwrap_or(&0);
        freq.insert(letter, count + 1);
    }

    let mut content = format!("Char\tAmt\n");

    for (key, value) in freq {
        let val = match key {
            '\n' => "\\n", // escape newline to be visible
            '\r' => "\\r", // escape carriage return to be visible
            '"' => "\\\"",
            _ => &key.to_string(),
        };
        content.push_str(&format!("{}\t{}\n", val, value));
    }
    content
}
