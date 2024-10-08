use crate::modules::number_utils;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PrimeCulator {
    mode: String,
    // #[serde(skip)] // This how you opt-out of serialization of a field
    value1: i32,
    value2: i32,
    value3: i32,
    result: String,
}

impl Default for PrimeCulator {
    fn default() -> Self {
        Self {
            mode: "empty!".to_owned(),
            value1: 1,
            value2: 1,
            value3: 0,
            result: "Result".to_owned(),
        }
    }
}

impl PrimeCulator {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.heading("PrimeCulator");

        egui::ComboBox
            ::from_id_salt("select")
            .selected_text(&self.mode)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.mode, "GCD".to_string(), "GCD");
                ui.selectable_value(&mut self.mode, "Prime numbers".to_string(), "Prime numbers");
            });
        ui.horizontal(|ui| {
            ui.label("a:");
            ui.add(egui::DragValue::new(&mut self.value1).range(0..=10000));
            ui.label("b:");
            ui.add(egui::DragValue::new(&mut self.value2).range(0..=10000));
            ui.label("c:");
            ui.add_enabled(
                if self.mode == "GCD" {
                    true
                } else {
                    false
                },
                egui::DragValue::new(&mut self.value3).range(0..=10000)
            );
        });

        if ui.button("compute").clicked() {
            match self.mode.as_str() {
                "GCD" => {
                    let mut res = number_utils::calculate_gcd(self.value1, self.value2);
                    if self.value3 != 0 {
                        res = number_utils::calculate_gcd(res, self.value3);
                    }
                    self.result = res.to_string();
                }
                "Prime numbers" => {
                    self.result = number_utils
                        ::find_prime_numbers(self.value1, self.value2)
                        .iter()
                        .map(|x| x.to_string() + ",")
                        .collect();
                }
                _ => {}
            }
        }

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading(&self.result.to_string());
        });
    }
}
