use super::rotor::Rotor;

pub fn enigma_cipher(text: String, start_positions: (i32, i32, i32)) -> String {
    let mut res = String::new();

    //8
    let mut rotor_l = Rotor::new("FKQHTLXOCBJSPDZRAMEWNIUYGV".to_string(), 1, start_positions.0);
    //2
    let mut rotor_m = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE".to_string(), 1, start_positions.1);
    //4
    let mut rotor_r = Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB".to_string(), 1, start_positions.2);

    //b
    let reflector = Rotor::new("YRUHQSLDPXNGOKMIEBFZCWVJAT".to_string(), 0, 0);

    let mut count = 0;

    for char in text.to_uppercase().chars() {
        let mut char = rotor_r.get(char);
        char = rotor_m.get(char);
        char = rotor_l.get(char);
        char = reflector.get(char);
        char = rotor_l.get_back(char);
        char = rotor_m.get_back(char);
        char = rotor_r.get_back(char);
        res.push(char);

        rotor_r.rotate();
        count += 1;

        if count % 26 == 0 {
            rotor_m.rotate();
        }

        rotor_l.rotate();
    }
    res
}
