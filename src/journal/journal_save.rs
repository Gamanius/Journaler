use std::{error::Error, fs, path::PathBuf};

use crate::journal::Journal;

#[derive(Default)]
pub struct JournalSave {

}

impl JournalSave {
    pub fn update(&mut self, ui: &mut egui::Ui, journal: &mut Journal) -> bool {
        let mut changed_name = false;
        ui.horizontal(|ui| {
            if ui.button("Speichern").clicked() {
                if journal.path == None {

                    if let Some(mut path) =  rfd::FileDialog::new()
                        .add_filter("Journals", &["jrl"])
                        .save_file() {
                            if path.extension().and_then(|e| e.to_str()) != Some("jrl") {
                                path = path.with_extension("jrl");
                            }

                            journal.path = Some(path);
                        }
                }

                if journal.path != None {
                    save_journal(&journal);
                }
            }

            if ui.button("Open").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Journals", &["jrl"])
                    .pick_file() {
                    let res = open_journal(&path);
                    match res {
                        Err(e) => println!("{}", e),
                        Ok(t) => *journal = t,
                    }
                    changed_name = true;
                }
            }
        });

        changed_name
    }
}

pub fn save_journal(j: &Journal) {
    let path = match &j.path {
        Some(p) => p,
        None => return,
    };

    let json = serde_json::to_string_pretty(&j).unwrap();
    let _ = fs::write(path, json);
}

pub fn open_journal(path: &PathBuf) -> Result<Journal, Box<dyn Error>> {
    let bytes = fs::read(path)?;
    let mut journal: Journal = serde_json::from_slice(&bytes)?;
    journal.path = Some(path.clone());
    Ok(journal)
}