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

pub fn double_permutation_encrypt(text: String, key_word1: String, key_word2: String) -> String {
    let algorithm = |
        indexes_row: &Vec<usize>,
        indexes_column: &Vec<usize>,
        letters: &mut Vec<&str>
    | {
        let mut res = String::new();

        for index_row in 0..indexes_row.len() {
            let pos_row = indexes_row
                .iter()
                .position(|x| x == &index_row)
                .unwrap();

            for index_column in 0..indexes_column.len() {
                let pos_column = indexes_column
                    .iter()
                    .position(|x| x == &index_column)
                    .unwrap();

                res.push_str(letters.get(pos_row * indexes_column.len() + pos_column).unwrap());
            }
        }

        res
    };

    double_permutation(text, key_word1, key_word2, algorithm)
}

pub fn double_permutation_decrypt(text: String, key_word1: String, key_word2: String) -> String {
    let algorithm = |
        indexes_row: &Vec<usize>,
        indexes_column: &Vec<usize>,
        letters: &mut Vec<&str>
    | {
        let mut res = String::new();

        for index_row in 0..indexes_row.len() {
            let pos_row = indexes_row.get(index_row).unwrap();

            for index_column in 0..indexes_column.len() {
                let pos_column = indexes_column.get(index_column).unwrap();

                res.push_str(letters.get(pos_row * indexes_column.len() + pos_column).unwrap());
            }
        }

        res
    };
    double_permutation(text, key_word1, key_word2, algorithm)
}

fn double_permutation<F>(text: String, key_word1: String, key_word2: String, algorithm: F) -> String
    where F: Fn(&Vec<usize>, &Vec<usize>, &mut Vec<&str>) -> String
{
    let mut res = String::new();

    //keyword 1 is column, keyword 2 is row
    let key_word1 = key_word1.to_lowercase();
    let key_word2 = key_word2.to_lowercase();

    let indexes_column = get_indexes(key_word1);
    let indexes_row = get_indexes(key_word2);

    let block_size = indexes_column.len() * indexes_row.len();

    for block in split_into_blocks(text, block_size) {
        let mut iter: Vec<&str> = block.graphemes(true).collect();

        while iter.len() < block_size {
            iter.push(" ");
        }

        res.push_str(&algorithm(&indexes_row, &indexes_column, &mut iter));
    }

    res
}

fn get_indexes(key_word: String) -> Vec<usize> {
    let mut keys: Vec<&str> = key_word.graphemes(true).collect();

    keys.sort();

    key_word
        .graphemes(true)
        .map(|x| {
            let index = keys
                .iter()
                .position(|y| y == &x)
                .unwrap();

            keys[index] = "";

            index
        })
        .collect::<Vec<usize>>()
}

#[test]
fn get_indexes_test() {
    assert_eq!(get_indexes("abc".to_string()), vec![0, 1, 2]);
    assert_eq!(get_indexes("cba".to_string()), vec![2, 1, 0]);
    assert_eq!(get_indexes("things".to_string()), vec![5, 1, 2, 3, 0, 4]);
    assert_eq!(get_indexes("some".to_string()), vec![3, 2, 1, 0]);
    assert_eq!(get_indexes("2310".to_string()), vec![2, 3, 1, 0]);
    assert_eq!(get_indexes("423051".to_string()), vec![4, 2, 3, 0, 5, 1]);
}

#[test]
fn double_permutation_test() {
    let keyword1 = "1320".to_string();
    let keyword2 = "423051".to_string();
    let text = "test message with_different symb01s for \n complex testing".to_string();
    let encrypted = double_permutation_encrypt(text.clone(), keyword1.clone(), keyword2.clone());
    let decrypted = double_permutation_decrypt(encrypted.clone(), keyword1, keyword2)
        .trim()
        .to_string();
    assert_eq!(decrypted, text);
}

#[test]
fn route_permutation_test() {
    let text = "test message with_different symb01s for \n complex testing".to_string();
    let encrypted = route_permutation_encrypt(text.clone());
    let decrypted = route_permutation_decrypt(encrypted.clone())
        .trim()
        .to_string();
    assert_eq!(decrypted, text);
}

