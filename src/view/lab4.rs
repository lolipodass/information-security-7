use egui::DragValue;

use crate::modules::enigma::enigma_cipher::enigma_cipher;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EnigmaCipher {
    input: String,
    result: String,
    positions: (i32, i32, i32),
    rotations: (i32, i32, i32),
}

impl Default for EnigmaCipher {
    fn default() -> Self {
        Self {
            input: "empty!".to_owned(),
            result: "Result".to_owned(),
            positions: (0, 0, 0),
            rotations: (0, 0, 0),
        }
    }
}

impl EnigmaCipher {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.heading("lab4");

        ui.text_edit_singleline(&mut self.input);

        ui.horizontal(|ui| {
            ui.label("positions: ");
            ui.add(
                DragValue::new(&mut self.positions.0)
                    .speed(1)
                    .range(0..=25)
            );
            ui.add(DragValue::new(&mut self.positions.1).range(0..=25));
            ui.add(DragValue::new(&mut self.positions.2).range(0..=25));
        });

        ui.horizontal(|ui| {
            ui.label("rotations: ");
            ui.add(
                DragValue::new(&mut self.rotations.0)
                    .speed(1)
                    .range(0..=25)
            );
            ui.add(DragValue::new(&mut self.rotations.1).range(0..=25));
            ui.add(DragValue::new(&mut self.rotations.2).range(0..=25));
        });
        if ui.button("compute").clicked() {
            self.result = enigma_cipher(self.input.clone(), self.rotations, self.positions);
        }

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading(&self.result.to_string());
        });
    }
}
