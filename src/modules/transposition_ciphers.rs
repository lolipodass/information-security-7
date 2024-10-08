use unicode_segmentation::UnicodeSegmentation;

use crate::utils::split_into_blocks::split_into_blocks;

pub fn route_permutation_encrypt(text: String) -> String {
    let order: [usize; 16] = [1, 2, 5, 9, 6, 3, 4, 7, 10, 13, 14, 11, 8, 12, 15, 16];
    route_permutation(text, order)
}

pub fn route_permutation_decrypt(text: String) -> String {
    let order: [usize; 16] = [1, 2, 6, 7, 3, 5, 8, 13, 4, 9, 12, 14, 10, 11, 15, 16];
    route_permutation(text, order)
}

fn route_permutation(text: String, order: [usize; 16]) -> String {
    let mut res = String::new();

    for block in split_into_blocks(text, 16) {
        let mut buff = String::new();

        let iter: Vec<&str> = block.graphemes(true).collect();

        for index in order.iter() {
            match iter.get(index - 1) {
                Some(x) => buff.push_str(x),
                None => {
                    buff.push(' ');
                }
            }
        }

        res.push_str(&buff);
    }

    res
}
