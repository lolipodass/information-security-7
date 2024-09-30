use std::collections::HashMap;

pub fn count_frequency(s: String) -> String {
    let mut freq = HashMap::new();

    for letter in s.chars() {
        let count = freq.get(&letter).unwrap_or(&0);
        freq.insert(letter, count + 1);
    }

    let mut content = format!("Char,Amt\n");

    for (key, value) in freq {
        let val = match key {
            '\n' => "\\n",
            '\r' => "\\r",
            _ => &key.to_string(),
        };
        content.push_str(&format!("{},{}\n", val, value));
    }
    content
}
