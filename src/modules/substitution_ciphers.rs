pub fn caesars(text: String, alphabet: &str, shift: usize) -> String {
    let length = alphabet.chars().count();
    let mut res = String::new();

    for letter in text.chars() {
        let index = alphabet.find(letter).ok_or(0).unwrap_or(0);
        res.push(
            alphabet
                .chars()
                .nth((index + shift) % length)
                .ok_or("")
                .unwrap()
        );
    }

    res
}
