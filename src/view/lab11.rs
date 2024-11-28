use egui_extras::Column;

use crate::modules::ecc::{ curve, point::Point };

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EEC {
    a: i64,
    b: i64,
    n: u64,
    k: u64,
    l: u64,
    input: String,
    points: Vec<Point>,
    points_computed: Vec<(Point, Point, Point, Point, Point)>,
}

impl Default for EEC {
    fn default() -> Self {
        Self {
            input: String::new(),
            a: -1,
            b: 1,
            n: 751,
            k: 8,
            l: 5,
            points: Vec::new(),
            points_computed: Vec::new(),
        }
    }
}

impl EEC {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.heading("lab11");

        ui.text_edit_singleline(&mut self.input);

        //inpu
        ui.horizontal(|ui| {
            ui.label("a: ");
            ui.add(egui::DragValue::new(&mut self.a));
            ui.label("b: ");
            ui.add(egui::DragValue::new(&mut self.b));
            ui.label("n: ");
            ui.add(egui::DragValue::new(&mut self.n));
            ui.label("k: ");
            ui.add(egui::DragValue::new(&mut self.k));
            ui.label("l: ");
            ui.add(egui::DragValue::new(&mut self.l));
        });
        ui.add_space(10.0);

        if ui.button("compute").clicked() {
            self.compute();
        }

        ui.separator();

        egui_extras::TableBuilder
            ::new(ui)
            .id_salt("lab11")
            .column(Column::initial(70.0))
            .column(Column::initial(70.0))
            .column(Column::initial(70.0))
            .column(Column::initial(90.0))
            .column(Column::remainder())
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Point");
                });
                header.col(|ui| {
                    ui.heading("kP");
                });
                header.col(|ui| {
                    ui.heading("P + Q");
                });
                header.col(|ui| {
                    ui.heading("kP + lQ - R");
                });
                header.col(|ui| {
                    ui.heading("P - Q + R");
                });
            })
            .body(|body| {
                body.rows(20.0, self.points_computed.len(), |mut row| {
                    let point = self.points_computed[row.index()];
                    row.col(|ui| {
                        ui.label(point.0.to_string());
                    });
                    row.col(|ui| {
                        ui.label(point.1.to_string());
                    });
                    row.col(|ui| {
                        ui.label(point.2.to_string());
                    });
                    row.col(|ui| {
                        ui.label(point.3.to_string());
                    });
                    row.col(|ui| {
                        ui.label(point.4.to_string());
                    });
                });
            });
    }
    pub fn compute(&mut self) {
        let eec = curve::EpilepticCurve::new(self.n, self.a, self.b);
        self.points = eec.find_point_in_range(0, self.n as i64);

        let mut points_computed = Vec::new();
        for p in &self.points {
            let q = eec.scalar(*p, self.k);
            let r = eec.add(*p, q);
            let s = eec.add(eec.add(q, eec.scalar(q, self.l)), -r);
            let g = eec.add(eec.add(*p, -q), r);

            points_computed.push((*p, q, r, s, g));
        }

        self.points_computed = points_computed;
    }
}
