use unicode_segmentation::UnicodeSegmentation;

pub fn caesars(text: String, alphabet: &str, shift: i32) -> String {
    let length = alphabet.chars().count();
    let mut res = String::new();

    for letter in text.to_lowercase().chars() {
        let index = alphabet.graphemes(true).position(|x| x == &letter.to_string());
        match index {
            Some(idx) => {
                res.push(
                    alphabet
                        .chars()
                        .nth(((idx as i32) + shift).rem_euclid(length as i32) as usize)
                        .ok_or("")
                        .unwrap()
                );
            }
            None => {
                res.push(letter);
            }
        }
    }

    res
}

pub fn trisemus(text: String, alphabet: String, key_word: &str, cols: i32) -> String {
    let length = alphabet.chars().count();
    let mut res = String::new();

    let mut table = String::new();

    for letter in key_word.chars() {
        if table.find(letter).is_none() {
            table.push(letter);
        }
    }

    for letter in alphabet.chars() {
        if table.find(letter).is_none() {
            table.push(letter);
        }
    }

    for letter in text.to_lowercase().chars() {
        let index = table.graphemes(true).position(|x| x == &letter.to_string());
        match index {
            Some(idx) => {
                res.push(
                    table
                        .chars()
                        .nth(((idx as i32) + cols).rem_euclid(length as i32) as usize)
                        .ok_or("")
                        .unwrap()
                );
            }
            None => {
                res.push(letter);
            }
        }
    }

    res
}
