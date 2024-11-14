use std::fs;

use crate::modules::md5::md5::md5;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct HashMD5 {
    input: Vec<u8>,
    result: String,
}

impl Default for HashMD5 {
    fn default() -> Self {
        Self {
            input: Vec::new(),
            result: "Result".to_owned(),
        }
    }
}

impl HashMD5 {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.heading("lab9");

        let mut input_str = String::from_utf8_lossy(&self.input).to_string();
        ui.text_edit_singleline(&mut input_str);
        self.input = input_str.into_bytes();

        if ui.button("select file").clicked() {
            self.select_file();
        }

        ui.add_space(10.0);

        if ui.button("compute").clicked() {
            self.result = hex::encode(md5(&self.input.clone()));
        }

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading(&self.result.to_string());
        });
    }
    fn select_file(&mut self) {
        let file_dir = rfd::FileDialog
            ::new()
            .set_title("select file")
            .set_directory("C:\\Users\\joper\\Desktop\\Flesha\\rust\\safety2\\Primeculator")
            .pick_file();

        let input = fs::read(file_dir.as_ref().unwrap());

        match input {
            Ok(val) => {
                self.input = val;
            }
            Err(_) => {
                self.input = Vec::new();
            }
        }
    }
}
