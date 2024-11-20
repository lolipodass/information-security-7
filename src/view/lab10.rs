use egui_extras::Column;
use num_bigint::BigUint;

use crate::modules::{ el_gamal::ElGamal, rsa::RSA, schnorr_signature::SchnorrSignature };

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Signing {
    input: String,
    rsa_res: bool,
    el_gamal_res: bool,
    schnorr_res: bool,
    rsa_key: Vec<u8>,
    el_gamal_key: (BigUint, BigUint),
    schnorr_key: (BigUint, BigUint),
}

impl Default for Signing {
    fn default() -> Self {
        Self {
            input: String::new(),
            rsa_res: false,
            el_gamal_res: false,
            schnorr_res: false,
            rsa_key: Vec::new(),
            el_gamal_key: (BigUint::default(), BigUint::default()),
            schnorr_key: (BigUint::default(), BigUint::default()),
        }
    }
}

impl Signing {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.heading("lab10");

        ui.text_edit_singleline(&mut self.input);

        ui.add_space(10.0);

        if ui.button("compute").clicked() {
            self.compute();
        }

        ui.separator();

        egui_extras::TableBuilder
            ::new(ui)
            .id_salt("lab10")
            .column(Column::initial(80.0))
            .column(Column::initial(80.0))
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("name");
                });
                header.col(|ui| {
                    ui.heading("correct");
                });
                header.col(|ui| {
                    ui.heading("key");
                });
            })
            .body(|mut body| {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.heading("RSA");
                    });

                    row.col(|ui| {
                        ui.label(self.rsa_res.to_string());
                    });
                    row.col(|ui| {
                        ui.label(format!("{:?}", self.rsa_key));
                    });
                });
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.heading("El Gamal");
                    });

                    row.col(|ui| {
                        ui.label(self.el_gamal_res.to_string());
                    });
                    row.col(|ui| {
                        ui.label(format!("{:?}", self.el_gamal_key));
                    });
                });
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.heading("Schnorr");
                    });

                    row.col(|ui| {
                        ui.label(self.schnorr_res.to_string());
                    });
                    row.col(|ui| {
                        ui.label(format!("{:?}", self.schnorr_key));
                    });
                });
            });
    }
    pub fn compute(&mut self) {
        let rsa = RSA::new(1024);
        self.rsa_key = rsa.sign(self.input.as_bytes());
        self.rsa_res = rsa.verify(&self.input.as_bytes(), &self.rsa_key);

        let el_gamal = ElGamal::new(100);
        self.el_gamal_key = el_gamal.sign(self.input.as_bytes());
        self.el_gamal_res = el_gamal.verify(&self.input.as_bytes(), self.el_gamal_key.clone());

        let schnorr = SchnorrSignature::new(20);
        self.schnorr_key = schnorr.sign(self.input.as_bytes());
        self.schnorr_res = schnorr.verify(self.input.as_bytes(), self.schnorr_key.clone());
    }
}
