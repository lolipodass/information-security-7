use primeculator::view::{
    lab1::PrimeCulator,
    lab2::SubCipher,
    lab3::TransCipher,
    lab4::EnigmaCipher,
    lab5::DESChipher,
    lab6::PRSGenerators,
    lab7::KnapsackView,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PrimeCulatorBase {
    lab1: PrimeCulator,
    lab2: SubCipher,
    lab3: TransCipher,
    lab4: EnigmaCipher,
    lab5: DESChipher,
    lab6: PRSGenerators,
    lab7: KnapsackView,
}

impl Default for PrimeCulatorBase {
    fn default() -> Self {
        Self {
            lab1: PrimeCulator::default(),
            lab2: SubCipher::default(),
            lab3: TransCipher::default(),
            lab4: EnigmaCipher::default(),
            lab5: DESChipher::default(),
            lab6: PRSGenerators::default(),
            lab7: KnapsackView::default(),
        }
    }
}

impl PrimeCulatorBase {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for PrimeCulatorBase {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
            ui.add_space(1.0);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.collapsing("lab 1", |ui| { self.lab1.update(ui) });
            ui.collapsing("lab 2", |ui| { self.lab2.update(ui) });
            ui.collapsing("lab 3", |ui| { self.lab3.update(ui) });
            ui.collapsing("lab 4", |ui| { self.lab4.update(ui) });
            ui.collapsing("lab 5", |ui| { self.lab5.update(ui) });
            ui.collapsing("lab 6", |ui| { self.lab6.update(ui) });
            ui.collapsing("lab 7", |ui| { self.lab7.update(ui) });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
