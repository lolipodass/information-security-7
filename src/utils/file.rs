use std::path::PathBuf;

/// Saves a string to a file
///
/// # Arguments
/// * `content`: The string to save
/// * `file_dir`: The path to the file
/// * `filename`: The name of the file
pub fn save(content: &String, file_dir: &Option<PathBuf>, filename: &str) {
    if let Some(file) = file_dir {
        std::fs::write(file.join(filename), content).unwrap();
    }
}

/// Reads a file and returns its content
///
/// # Arguments
/// * `file_dir`: The path to the file
/// # Returns
/// The content of the file
pub fn read(file_dir: &PathBuf) -> String {
    if let Ok(content) = std::fs::read_to_string(file_dir) {
        content
    } else {
        "Ошибка при чтении файла".to_string()
    }
}
