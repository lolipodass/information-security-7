use crate::modules::substitution_ciphers::{ caesars, trisemus };

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SubCipher {
    mode: String,
    input: String,
    shift: i32,
    encrypted: String,
    decrypted: String,
    alphabet: String,
    alphabet_length: usize,
}

impl Default for SubCipher {
    fn default() -> Self {
        let alphabet = "абвгдежзийклмнопрстуфхцчшщъыьэюя".to_owned();
        // let alphabet = "abcdefghijklmnopqrstuvwxyz .,;".to_owned();

        Self {
            mode: "empty!".to_owned(),
            input: "".to_owned(),
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

        ui.text_edit_singleline(&mut self.input);

        egui::ComboBox
            ::from_id_salt("select")
            .selected_text(&self.mode)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.mode, "caesars".to_string(), "caesars");
                ui.selectable_value(&mut self.mode, "trisemus".to_string(), "trisemus");
            });

        ui.separator();

        if self.mode == "caesars" {
            ui.horizontal(|ui| {
                ui.label("shift: ");
                ui.add(egui::DragValue::new(&mut self.shift).range(0..=self.alphabet_length));
            });
        }

        if ui.button("encrypt").clicked() {
            match self.mode.as_str() {
                "caesars" => {
                    self.encrypted = caesars(self.input.clone(), &self.alphabet, self.shift);
                    self.decrypted = caesars(self.encrypted.clone(), &self.alphabet, -self.shift);
                }
                "trisemus" => {
                    self.encrypted = trisemus(
                        self.input.clone(),
                        self.alphabet.clone(),
                        "цезарь",
                        4
                    );
                    self.decrypted = trisemus(
                        self.encrypted.clone(),
                        self.alphabet.clone(),
                        "цезарь",
                        -4
                    );
                }
                _ => {}
            }
        }
        ui.label(format!("encrypted: {}", &self.encrypted));
        ui.label(format!("decrypted: {}", &self.decrypted));

        ui.horizontal(|ui| {
            ui.label(&self.input);
            ui.separator();
            ui.label(&self.decrypted);
        });
    }
}
