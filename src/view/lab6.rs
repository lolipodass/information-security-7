use crate::modules::generators::bbs;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PRSGenerators {
    input: String,
    key: String,
    n: u16,
    seed: u16,
    amount: u32,
    result: String,
}

impl Default for PRSGenerators {
    fn default() -> Self {
        Self {
            input: String::new(),
            key: String::new(),
            n: 2,
            seed: 1,
            amount: 10,
            result: Vec,
        }
    }
}

impl PRSGenerators {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.heading("lab6");
        ui.horizontal(|ui| {
            ui.label("n:");
            ui.add(egui::DragValue::new(&mut self.n));
            ui.label("seed:");
            ui.add(egui::DragValue::new(&mut self.seed));
            ui.label("amount:");
            ui.add(egui::DragValue::new(&mut self.amount).range(0..=10000));
        });

        if ui.button("bbs").clicked() {
            self.result = bbs(self.n, self.seed, self.amount);
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading(&self.result.to_string());
        });
    }
}
