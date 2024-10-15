use std::char;

pub struct Rotor {
    shift: i32,
    shift_count: i32,
    alphabet: String,
    initial_alphabet: String,
}

impl Rotor {
    pub fn new(alphabet: String, shift: i32, start_position: i32) -> Self {
        Self {
            shift_count: start_position,
            shift,
            alphabet: alphabet.to_uppercase(),
            initial_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
        }
    }
    pub fn get(&self, char: char) -> char {
        let char = char.to_ascii_uppercase();
        let position = self.initial_alphabet
            .chars()
            .position(|x| x == char)
            .unwrap();

        let index = (position as i32) + self.shift_count;

        self.alphabet
            .chars()
            .nth(index.rem_euclid(self.alphabet.len() as i32) as usize)
            .unwrap()
    }

    pub fn get_back(&self, char: char) -> char {
        let char = char.to_ascii_uppercase();
        let position = self.alphabet
            .chars()
            .position(|x| x == char)
            .unwrap();

        let index = (position as i32) - self.shift_count;

        self.initial_alphabet
            .chars()
            .nth(index.rem_euclid(self.initial_alphabet.len() as i32) as usize)
            .unwrap()
    }
    pub fn rotate(&mut self) {
        self.shift_count += self.shift;
    }
}
