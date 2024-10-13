use std::path::PathBuf;

use egui::ScrollArea;

use crate::{
    modules::transposition_ciphers::{
        double_permutation_decrypt,
        double_permutation_encrypt,
        route_permutation_decrypt,
        route_permutation_encrypt,
    },
    utils::{ count_frequency::count_frequency, file::{ read, save } },
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TransCipher {
    mode: String,
    input: Option<String>,
    file_dir: Option<PathBuf>,
    key_word1: String,
    key_word2: String,
    encrypted: String,
    decrypted: String,
    #[serde(skip)]
    alphabet: String,
    #[serde(skip)]
    alphabet_length: usize,
}

impl Default for TransCipher {
    fn default() -> Self {
        let alphabet = "abcdefghijklmnopqrstuvwxyzäöüß .,;".to_owned();

        Self {
            mode: "empty!".to_owned(),
            input: None,
            file_dir: None,
            key_word1: "".to_owned(),
            key_word2: "".to_owned(),
            encrypted: "".to_owned(),
            decrypted: "".to_owned(),
            alphabet: alphabet.clone(),
            alphabet_length: alphabet.chars().count(),
        }
    }
}

impl TransCipher {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.heading("lab3");

        ui.horizontal(|ui| {
            egui::ComboBox
                ::from_id_salt("select")
                .selected_text(&self.mode)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.mode,
                        "single".to_string(),
                        "route transposition cipher"
                    );
                    ui.selectable_value(
                        &mut self.mode,
                        "double".to_string(),
                        "double transposition"
                    );
                });

            if ui.button("select file").clicked() {
                self.select_file();
            }
        });

        ui.separator();

        if let Some(content) = &self.input {
            ui.label(content.len().to_string() + " bytes");
        }

        ui.text_edit_singleline(self.input.as_mut().unwrap());

        if self.mode == "double" {
            ui.horizontal(|ui| {
                ui.label("keyword1: ");
                ui.text_edit_singleline(&mut self.key_word1);
            });
            ui.horizontal(|ui| {
                ui.label("keyword2: ");
                ui.text_edit_singleline(&mut self.key_word2);
            });
        }

        ui.horizontal(|ui| {
            if self.input.is_some() {
                if ui.button("compute").clicked() {
                    self.compute();
                }
            }
            if ui.button("save").clicked() {
                save(&self.encrypted, &self.file_dir, "encrypted.txt");
                save(&self.decrypted, &self.file_dir, "decrypted.txt");
                save(
                    &count_frequency(self.encrypted.clone()),
                    &self.file_dir,
                    "encrypted_freq.csv"
                );
                save(
                    &count_frequency(self.decrypted.clone()),
                    &self.file_dir,
                    "decrypted_freq.csv"
                );
            }
        });

        ScrollArea::both()
            .auto_shrink(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(self.input.clone().unwrap_or("file not selected".to_owned()));
                    ui.separator();
                    ui.label(&self.encrypted);
                    ui.separator();
                    ui.label(&self.decrypted);
                });
            });
    }

    fn compute(&mut self) {
        match self.mode.as_str() {
            "single" => {
                self.encrypted = route_permutation_encrypt(self.input.clone().unwrap());
                self.decrypted = route_permutation_decrypt(self.encrypted.clone());
            }
            "double" => {
                self.encrypted = double_permutation_encrypt(
                    self.input.clone().unwrap(),
                    self.key_word1.clone(),
                    self.key_word2.clone()
                );
                self.decrypted = double_permutation_decrypt(
                    self.encrypted.clone(),
                    self.key_word1.clone(),
                    self.key_word2.clone()
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

        self.input = Some(read(self.file_dir.as_ref().unwrap()));
        self.file_dir = self.file_dir.clone().and_then(|p| p.parent().map(PathBuf::from));
    }
}
