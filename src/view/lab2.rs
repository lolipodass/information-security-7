use std::path::PathBuf;

use egui::ScrollArea;

use crate::{
    modules::substitution_ciphers::{ caesars, trisemus },
    utils::count_frequency::count_frequency,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SubCipher {
    mode: String,
    input: Option<String>,
    file_dir: Option<PathBuf>,
    shift: i32,
    encrypted: String,
    decrypted: String,
    #[serde(skip)]
    alphabet: String,
    #[serde(skip)]
    alphabet_length: usize,
}

impl Default for SubCipher {
    fn default() -> Self {
        let alphabet = "abcdefghijklmnopqrstuvwxyzäöüß .,;".to_owned();

        Self {
            mode: "empty!".to_owned(),
            input: None,
            file_dir: None,
            shift: 1,
            encrypted: "".to_owned(),
            decrypted: "".to_owned(),
            alphabet: alphabet.clone(),
            alphabet_length: alphabet.chars().count(),
        }
    }
}

impl SubCipher {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.heading("lab2 ");

        ui.horizontal(|ui| {
            egui::ComboBox
                ::from_id_salt("select")
                .selected_text(&self.mode)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.mode, "caesars".to_string(), "caesars");
                    ui.selectable_value(&mut self.mode, "trisemus".to_string(), "trisemus");
                });

            if ui.button("Выбрать файл").clicked() {
                self.select_file();
            }
        });

        ui.separator();

        if let Some(content) = &self.input {
            ui.label(content.len().to_string() + " bytes");
        }

        if self.mode == "caesars" {
            ui.horizontal(|ui| {
                ui.label("shift: ");
                ui.add(egui::DragValue::new(&mut self.shift).range(0..=self.alphabet_length));
            });
        }
        ui.horizontal(|ui| {
            if self.input.is_some() {
                if ui.button("compute").clicked() {
                    self.compute();
                }
            }
            if ui.button("save").clicked() {
                self.save(&self.encrypted, "encrypted.txt");
                self.save(&self.decrypted, "decrypted.txt");
                self.save(&count_frequency(self.encrypted.clone()), "encrypted_freq.csv");
                self.save(&count_frequency(self.decrypted.clone()), "decrypted_freq.csv");
            }
        });

        ScrollArea::both()
            .auto_shrink(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(self.input.clone().unwrap_or("file not selected".to_owned()));
                    ui.separator();
                    ui.label(&self.decrypted);
                });
            });
    }

    fn save(&self, content: &String, filename: &str) {
        if let Some(file) = &self.file_dir {
            std::fs::write(file.join(filename), content).unwrap();
        }
    }

    fn compute(&mut self) {
        match self.mode.as_str() {
            "caesars" => {
                self.encrypted = caesars(self.input.clone().unwrap(), &self.alphabet, self.shift);
                self.decrypted = caesars(self.encrypted.clone(), &self.alphabet, -self.shift);
            }
            "trisemus" => {
                self.encrypted = trisemus(
                    self.input.clone().unwrap(),
                    self.alphabet.clone(),
                    "enigma",
                    4
                );
                self.decrypted = trisemus(
                    self.encrypted.clone(),
                    self.alphabet.clone(),
                    "enigma",
                    -4
                );
            }
            _ => {}
        }
    }

    fn select_file(&mut self) {
        self.file_dir = rfd::FileDialog
            ::new()
            .add_filter("text", &["txt", "rs", "json", "toml", "md"])
            .set_title("Выберите файл")
            .set_directory("C:\\Users\\joper\\Desktop\\Flesha\\rust\\safety2\\Primeculator")
            .pick_file();

        if let Some(file) = &self.file_dir {
            if let Ok(content) = std::fs::read_to_string(file.as_path()) {
                self.input = Some(content);
                self.file_dir = Some(file.parent().unwrap().to_path_buf());
            } else {
                self.input = Some("Ошибка при чтении файла".to_string());
            }
        }
    }
}
