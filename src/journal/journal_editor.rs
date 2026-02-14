use std::f32;

use chrono::NaiveDate;

use crate::journal::{JournalDay, JournalEntry};

#[derive(Default)]
pub struct JournalEditor {
    current_string: String,
}

impl JournalEditor {
    pub fn update(&mut self, ui: &mut egui::Ui, journalday: &mut JournalDay, date: NaiveDate) {
        let now = chrono::Local::now().naive_local().time();
        ui.label(egui::RichText::new(date.format("%d.%m.%Y").to_string()).heading().strong());
        ui.separator();
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {

            ui.horizontal(|ui| {
                ui.label(now.format("%H:%M:%S").to_string());
                
                let res = ui.add(egui::TextEdit::singleline(&mut self.current_string).hint_text("New Entry").desired_width(f32::INFINITY));
                if res.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    journalday.entries.push(JournalEntry { time: now, entry: std::mem::take(&mut self.current_string) });
                    res.request_focus();
                }
            });
            
            ui.separator();
            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for entry in journalday.entries.iter() {
                        ui.horizontal(|ui| {
                            ui.label(entry.time.format("%H:%M").to_string());
                            ui.add(egui::Label::new(&entry.entry).wrap());
                        });
                    }
                });
            });



        });
    }
}