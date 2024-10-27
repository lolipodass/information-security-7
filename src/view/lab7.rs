use crate::{ modules::knapsack_cipher::KnapsackCipher, utils::conversions::vec_u128_to_u8 };

use base64::prelude::*;
use egui_extras::Column;
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct KnapsackView {
    input: String,
    knapsack: KnapsackCipher,
    amount: u8,
    mode: String,
    encrypted: Vec<u128>,
    result: Vec<u8>,
}

impl Default for KnapsackView {
    fn default() -> Self {
        Self {
            input: String::new(),
            amount: 10,
            mode: "plain".to_string(),
            knapsack: KnapsackCipher::default(),
            encrypted: Vec::new(),
            result: Vec::new(),
        }
    }
}

impl KnapsackView {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.heading("lab7");

        ui.add_space(10.0);
        egui::ComboBox
            ::from_id_salt("select")
            .selected_text(&self.mode)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.mode, "plain".to_string(), "Plain text");
                ui.selectable_value(&mut self.mode, "Base64".to_string(), "Base64");
            });
        ui.add_space(10.0);

        egui::Grid
            ::new("grid")
            .min_col_width(100.0)
            .show(ui, |ui| {
                ui.label("size:");
                ui.add(egui::DragValue::new(&mut self.amount).range(1..=8));
                ui.end_row();
                ui.label("input:");
                ui.add(egui::TextEdit::singleline(&mut self.input));
            });
        ui.add_space(10.0);
        if ui.button("clear").clicked() {
            self.result = Vec::new();
            self.encrypted = Vec::new();
            self.input = String::new();
            self.knapsack = KnapsackCipher::new(self.amount);
        }
        if ui.button("compute").clicked() {
            match self.mode.as_str() {
                "plain" => {
                    self.encrypted = self.knapsack.encrypt(self.input.clone().into_bytes());
                    self.result = self.knapsack.decrypt(self.encrypted.clone());
                }
                "Base64" => {
                    let base = BASE64_STANDARD.encode(self.input.clone());
                    self.encrypted = self.knapsack.encrypt(base.as_bytes().to_vec());

                    self.result = self.knapsack.decrypt(self.encrypted.clone());
                }
                _ => {}
            }
        }
        egui_extras::TableBuilder
            ::new(ui)
            .column(Column::initial(100.0).resizable(true))
            .column(Column::initial(100.0).resizable(true))
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("input");
                });
                header.col(|ui| {
                    ui.heading("encrypted");
                });
                header.col(|ui| {
                    ui.heading("decrypted");
                });
            })
            .body(|mut body| {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label(String::from_utf8_lossy(&self.input.clone().into_bytes()));
                    });

                    row.col(|ui| {
                        ui.label(String::from_utf8_lossy(&vec_u128_to_u8(self.encrypted.clone())));
                    });
                    row.col(|ui| {
                        ui.label(String::from_utf8_lossy(&self.result));
                    });
                });
            });
    }
}
