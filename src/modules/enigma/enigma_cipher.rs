use super::rotor::Rotor;

pub fn enigma_cipher(
    text: String,
    rotations: (i32, i32, i32),
    start_positions: (i32, i32, i32)
) -> String {
    let mut res = String::new();

    let mut rotor_l = Rotor::new(super::rotor::RotorType::VIII, rotations.0, start_positions.0);
    let mut rotor_m = Rotor::new(super::rotor::RotorType::II, rotations.1, start_positions.1);
    let mut rotor_r = Rotor::new(super::rotor::RotorType::IV, rotations.2, start_positions.2);

    let reflector = Rotor::new(super::rotor::RotorType::ReflectorB, 0, 0);

    let mut rotation_count_m = 0;
    let mut rotation_count_r = 0;

    for char in text.to_uppercase().chars() {
        let mut char = rotor_r.get(char);
        char = rotor_m.get(char);
        char = rotor_l.get(char);
        char = reflector.get(char);
        char = rotor_l.get_back(char);
        char = rotor_m.get_back(char);
        char = rotor_r.get_back(char);
        res.push(char);

        rotation_count_m = rotate_if_needed(&mut rotor_l, rotations.0, rotation_count_m);
        rotation_count_r = rotate_if_needed(&mut rotor_m, rotations.1, rotation_count_r);
        //I set to 1 because it is already the rightmost rotor
        rotate_if_needed(&mut rotor_r, rotations.2, 1);
    }
    res
}

fn rotate_if_needed(rotor: &mut Rotor, rotation: i32, count: usize) -> usize {
    if rotation != 0 {
        rotor.rotate();
        count + 1
    } else if count % 26 == 0 {
        rotor.rotate();
        count + 1
    } else {
        count
    }
}

#[test]
fn test_enigma() {
    let text = "AAAFFАВАВ";
    let encrypted = enigma_cipher(text.to_string(), (1, 0, 1), (0, 0, 0));
    let decrypted = enigma_cipher(encrypted, (1, 0, 1), (0, 0, 0));
    assert_eq!(text, decrypted);
}
