use unicode_segmentation::UnicodeSegmentation;

/// Splits a given text into blocks of a given size, determined by the given
/// block_size parameter.
///
/// Args:
/// * `text`: The string to split into blocks
/// * `block_size`: The size of each block
///
/// Returns:
/// A vector of strings, each of which is a block of the input string
///
/// Example:
/// ```
/// use primeculator::utils::split_into_blocks::split_into_blocks;
/// let text = "Text with 😥 different elemeпts ";
/// let block_size = 4;
/// let blocks = split_into_blocks(text.to_string(), block_size);
///
/// assert_eq!(blocks, vec!["Text", " wit", "h 😥 ", "diff", "eren", "t el", "emeп", "ts  "]);
/// ```
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
    if buff.len() < block_size {
        buff.push(' ');
    }

    if !buff.is_empty() {
        res.push(buff);
    }

    res
}
