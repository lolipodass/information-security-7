use std::{ fs, path::PathBuf };

use crate::modules::des::des::{ decrypt_des, encrypt_des };

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DESChipher {
    file_dir: Option<std::path::PathBuf>,
    input: Vec<u8>,
    key1: String,
    key2: String,
    key3: String,
}

impl Default for DESChipher {
    fn default() -> Self {
        Self {
            file_dir: None,
            input: Vec::new(),
            key1: "key1".to_owned(),
            key2: "key2".to_owned(),
            key3: "key3".to_owned(),
        }
    }
}

impl DESChipher {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        ui.heading("lab5");

        ui.label("key 1");
        ui.text_edit_singleline(&mut self.key1);
        ui.label("key 2");
        ui.text_edit_singleline(&mut self.key2);
        ui.label("key 3");
        ui.text_edit_singleline(&mut self.key3);

        ui.horizontal(|ui| {
            if ui.button("select file").clicked() {
                self.select_file();
            }
            if ui.button("read").clicked() {
                self.read();
            }

            if ui.button("compute").clicked() {
                if self.file_dir.is_some() {
                    let des_e = self.encrypt(
                        &self.input.clone(),
                        self.key1.clone(),
                        "encrypted1.txt"
                    );
                    let des_ee = self.encrypt(&des_e, self.key2.clone(), "encrypted2.txt");
                    let des_eee = self.encrypt(&des_ee, self.key3.clone(), "encrypted3.txt");

                    let des_d = self.decrypt(&des_eee, self.key3.clone(), "decrypted3.txt");
                    let des_dd = self.decrypt(&des_d, self.key2.clone(), "decrypted2.txt");
                    self.decrypt(&des_dd, self.key1.clone(), "decrypted1.txt");
                }
            }
        });
    }
    fn encrypt(&mut self, data: &Vec<u8>, key: String, filename: &str) -> Vec<u8> {
        let res = encrypt_des(data, &key.into_bytes().to_vec());
        self.save(filename, res.clone());

        res
    }

    fn decrypt(&mut self, data: &Vec<u8>, key: String, filename: &str) -> Vec<u8> {
        let res = decrypt_des(data, &key.into_bytes().to_vec());
        self.save(filename, res.clone());

        res
    }

    fn select_file(&mut self) {
        self.file_dir = rfd::FileDialog
            ::new()
            .add_filter("text", &["txt", "rs", "json", "toml", "md"])
            .set_title("Select file")
            .set_directory("C:\\Users\\joper\\Desktop\\Flesha\\rust\\safety2\\Primeculator")
            .pick_file();

        self.read();
    }
    fn read(&mut self) {
        if let Some(file_dir) = &self.file_dir {
            self.input = fs::read(file_dir).expect("bad file");
            self.file_dir = self.file_dir.clone().and_then(|p| p.parent().map(PathBuf::from));
        }
    }

    fn save(&mut self, filename: &str, input: Vec<u8>) {
        if let Some(file_dir) = &self.file_dir {
            let _ = fs::write(file_dir.join(filename), input);
        }
    }
}
