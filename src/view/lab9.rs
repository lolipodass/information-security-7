use crate::modules::md5::md5::md5;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct HashMD5 {
    input: String,
    result: String,
}

impl Default for HashMD5 {
    fn default() -> Self {
        Self {
            input: "empty!".to_owned(),
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

        ui.text_edit_singleline(&mut self.input);
        if ui.button("compute").clicked() {
            self.result = hex::encode(md5(&self.input.clone().into_bytes()));
        }

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading(&self.result.to_string());
        });
    }
}
