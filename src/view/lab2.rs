use crate::modules::substitution_ciphers::caesars;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SubCipher {
    input: String,
    shift: usize,
    result: String,
    alphabet: String,
    alphabet_length: usize,
}

impl Default for SubCipher {
    fn default() -> Self {
        let alphabet = "abcdefghijklmnopqrstuvwxyz".to_owned();

        Self {
            input: "".to_owned(),
            shift: 1,
            result: "Result".to_owned(),
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

        ui.horizontal(|ui| {
            ui.label("shift: ");
            ui.add(egui::DragValue::new(&mut self.shift).range(0..=self.alphabet_length));
        });

        if ui.button("compute").clicked() {
            self.result = caesars(self.input.clone(), &self.alphabet, self.shift);
        }
        ui.label(&self.result);
    }
}
