use unicode_segmentation::UnicodeSegmentation;

pub fn split_into_blocks(text: String, block_size: usize) -> Vec<String> {
    let mut res = Vec::new();

    let mut buff = String::new();
    let mut count: usize = 0;
    for char in text.graphemes(true) {
        if count >= block_size {
            res.push(buff.clone());
            buff.clear();
            count = 0;
        }
        buff.push_str(char);
        count += 1;
    }

    if !buff.is_empty() {
        res.push(buff);
    }

    res
}
