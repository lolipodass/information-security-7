use std::path::PathBuf;

pub fn save(content: &String, file_dir: &Option<PathBuf>, filename: &str) {
    if let Some(file) = file_dir {
        std::fs::write(file.join(filename), content).unwrap();
    }
}

pub fn read(file_dir: &PathBuf) -> String {
    if let Ok(content) = std::fs::read_to_string(file_dir) {
        content
    } else {
        "Ошибка при чтении файла".to_string()
    }
}
