use crate::modules::generators::{ bbs, rc4 };

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PRSGenerators {
    input: String,
    key: String,
    bbs_n: u16,
    rc4_n: u8,
    seed: u16,
    amount: u32,
    encrypted: Vec<u8>,
    result: Vec<u8>,
}

impl Default for PRSGenerators {
    fn default() -> Self {
        Self {
            input: String::new(),
            key: String::new(),
            bbs_n: 2,
            rc4_n: 1,
            seed: 1,
            amount: 10,
            encrypted: Vec::new(),
            result: Vec::new(),
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
        ui.label("BBS");
        ui.horizontal(|ui| {
            ui.label("n:");
            ui.add(egui::DragValue::new(&mut self.bbs_n));
            ui.label("seed:");
            ui.add(egui::DragValue::new(&mut self.seed));
            ui.label("amount:");
            ui.add(egui::DragValue::new(&mut self.amount).range(0..=10000));
        });

        if ui.button("BBS").clicked() {
            self.result = bbs(self.bbs_n, self.seed, self.amount);
        }

        ui.horizontal(|ui| {
            ui.label("key:");
            ui.text_edit_singleline(&mut self.key);
            ui.label("input:");
            ui.text_edit_singleline(&mut self.input);
            ui.label("n:");
            ui.add(egui::DragValue::new(&mut self.rc4_n));
        });
        if ui.button("RC4").clicked() {
            self.encrypted = rc4(
                self.input.clone().into_bytes(),
                self.rc4_n,
                self.key.clone().into_bytes()
            );

            self.result = rc4(self.encrypted.clone(), self.rc4_n, self.key.clone().into_bytes());
        }

        if ui.button("clear").clicked() {
            self.result = Vec::new();
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(String::from_utf8_lossy(&self.encrypted));
                ui.separator();
                ui.heading(String::from_utf8_lossy(&self.result));
            })
        });
    }
}
