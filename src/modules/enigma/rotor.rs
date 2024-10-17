use std::char;

pub enum RotorType {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    Beta,
    Gamma,
    ReflectorB,
    ReflectorC,
    ReflectorBDunn,
    ReflectorCDunn,
}

impl RotorType {
    pub fn to_string(&self) -> String {
        match self {
            RotorType::I => "EKMFLGDQVZNTOWYHXUSPAIBRCJ".to_string(),
            RotorType::II => "AJDKSIRUXBLHWTMCQGZNPYFVOE".to_string(),
            RotorType::III => "BDFHJLCPRTXVZNYEIWGAKMUSQO".to_string(),
            RotorType::IV => "ESOVPZJAYQUIRHXLNFTGKDCMWB".to_string(),
            RotorType::V => "VZBRGITYUPSDNHLXAWMJQOFECK".to_string(),
            RotorType::VI => "JPGVOUMFYQBENHZRDKASXLICTW".to_string(),
            RotorType::VII => "NZJHGRCXMYSWBOUFAIVLPEKQDT".to_string(),
            RotorType::VIII => "FKQHTLXOCBJSPDZRAMEWNIUYGV".to_string(),
            RotorType::Beta => "LEYJVCNIXWPBQMDRTAKZGFUHOS".to_string(),
            RotorType::Gamma => "FSOKANUERHMBTIYCWLQPZXVGJD".to_string(),
            RotorType::ReflectorB => "YRUHQSLDPXNGOKMIEBFZCWVJAT".to_string(),
            RotorType::ReflectorC => "FVPJIAOYEDRZXWGCTKUQSBNMHL".to_string(),
            RotorType::ReflectorBDunn => "ENKQAUYWJICOPBLMDXZVFTHRGS".to_string(),
            RotorType::ReflectorCDunn => "RDOBJNTKVEHMLFCWZAXGYIPSUQ".to_string(),
        }
    }
}

pub struct Rotor {
    shift: i32,
    shift_count: i32,
    alphabet: String,
    initial_alphabet: String,
}

impl Rotor {
    pub fn new(rotor_type: RotorType, shift: i32, start_position: i32) -> Self {
        Self {
            shift_count: start_position,
            shift,
            alphabet: rotor_type.to_string(),
            initial_alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
        }
    }
    pub fn get(&self, char: char) -> char {
        self.get_shifted(char, self.shift_count, &self.initial_alphabet, &self.alphabet)
    }

    pub fn get_back(&self, char: char) -> char {
        self.get_shifted(char, -self.shift_count, &self.alphabet, &self.initial_alphabet)
    }

    fn get_shifted(&self, char: char, shift: i32, source: &str, target: &str) -> char {
        let char = char.to_ascii_uppercase();

        match source.chars().position(|x| x == char) {
            Some(pos) => {
                let index = (pos as i32) - shift;

                target
                    .chars()
                    .nth(index.rem_euclid(target.len() as i32) as usize)
                    .expect("[Rotor] - Index out of bounds")
            }
            None => char,
        }
    }

    pub fn rotate(&mut self) {
        //add rem euclid to avoid overflow
        self.shift_count = (self.shift + self.shift_count).rem_euclid(26);
    }
}
