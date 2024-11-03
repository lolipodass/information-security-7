use std::{ collections::HashMap, hint::black_box, time::Instant };

use egui_extras::Column;
use egui_plot::{ Axis, AxisHints, Line, Plot, PlotPoints };
use num_bigint::{ BigUint, RandBigInt, ToBigUint };
use num_prime::RandPrime;

use crate::modules::{ el_gamal::ElGamal, rsa::RSA };

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AsymmetricCiphers {
    input: String,
    key_size: usize,
    encrypted_rsa: Vec<u8>,
    decrypted_rsa: Vec<u8>,
    encrypted_elgamal: Vec<u8>,
    decrypted_elgamal: Vec<u8>,
    graph: Vec<(u32, u128)>,
}

impl Default for AsymmetricCiphers {
    fn default() -> Self {
        Self {
            input: String::new(),
            key_size: 128,
            graph: Vec::new(),
            encrypted_rsa: Vec::new(),
            decrypted_rsa: Vec::new(),
            encrypted_elgamal: Vec::new(),
            decrypted_elgamal: Vec::new(),
        }
    }
}

impl AsymmetricCiphers {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.heading("lab8");
        ui.collapsing("Graph", |ui| {
            if ui.button("Generate Graph").clicked() {
                let mut graph: Vec<(u32, u128)> = self
                    .time_graph()
                    .into_iter()
                    .map(|(k, v)| (k, v))
                    .collect();
                graph.sort();
                self.graph = graph;

                println!("self.graph {:?}", self.graph);
            }

            let plot_points: PlotPoints = self.graph
                .iter()
                .map(|(x, time)| [*x as f64, *time as f64])
                .collect();

            let line = Line::new(plot_points);
            Plot::new("Execution Time Plot")
                .view_aspect(2.0)
                .y_axis_label("Time (ns)")
                .show(ui, |plot_ui| plot_ui.line(line));
        });

        ui.label("Input");
        ui.text_edit_singleline(&mut self.input);

        ui.label("Key");
        ui.add(egui::DragValue::new(&mut self.key_size).range(8..=300));

        ui.add_space(10.0);
        if ui.button("clear").clicked() {
            self.encrypted_rsa = Vec::new();
            self.decrypted_rsa = Vec::new();
            self.encrypted_elgamal = Vec::new();
            self.decrypted_elgamal = Vec::new();
            self.input = String::new();
            self.key_size = 128;
            self.graph = Vec::new();
        }

        if ui.button("compute").clicked() {
            self.compute();
        }

        egui_extras::TableBuilder
            ::new(ui)
            .column(Column::initial(80.0))
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
                        ui.heading("RSA");
                    });

                    row.col(|ui| {
                        ui.label(format!("{:?}", &self.encrypted_rsa));
                    });
                    row.col(|ui| {
                        ui.label(String::from_utf8_lossy(&self.decrypted_rsa));
                    });
                });

                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.heading("El Gamal");
                    });
                    row.col(|ui| {
                        ui.label(format!("{:?}", &self.encrypted_elgamal));
                    });
                    row.col(|ui| {
                        ui.label(String::from_utf8_lossy(&self.decrypted_rsa));
                    });
                });
            });
    }

    fn compute(&mut self) {
        let rsa = RSA::new(self.key_size);
        self.encrypted_rsa = rsa.encrypt(self.input.as_bytes());
        self.decrypted_rsa = rsa.decrypt(&self.encrypted_rsa);
        let elgamal = ElGamal::new(self.key_size);
        self.encrypted_elgamal = elgamal.encrypt(self.input.as_bytes());
        self.decrypted_elgamal = elgamal.decrypt(&self.encrypted_elgamal);
    }

    fn time_graph(&mut self) -> HashMap<u32, u128> {
        let mut res = HashMap::new();
        let mut rand = rand::thread_rng();

        let a = rand.gen_biguint_range(&(5).to_biguint().unwrap(), &(35).to_biguint().unwrap());
        let n: BigUint = rand.gen_prime(2048, None);

        let mut x_values = Vec::new();
        for _ in 0..10 {
            let x: BigUint = rand.gen_prime(100, None);
            x_values.push(x);
        }

        for (i, x) in x_values.iter().enumerate() {
            let now = Instant::now();

            let y = black_box(black_box(&a) * black_box(x)) % black_box(&n);
            black_box(y);
            res.insert(i as u32, now.elapsed().as_nanos());
        }

        res
    }
}
