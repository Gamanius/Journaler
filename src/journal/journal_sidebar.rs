use chrono::NaiveDate;

use crate::journal::Journal;

#[derive(Default)]
pub struct JournalSidebar {
    pub selected_day: NaiveDate,
}

impl JournalSidebar {
    pub fn update(&mut self, ui: &mut egui::Ui, journal: &mut Journal) {
        ui.horizontal(|ui| {
            ui.heading("All Notes");
        });
        let mut sorted_dates: Vec<NaiveDate> = journal.days.keys().cloned().collect();
        sorted_dates.sort(); // NaiveDate implements Ord

        for i in sorted_dates.iter() {
            if ui.button(i.format("%d.%m.%Y").to_string()).clicked() {
                self.selected_day = i.clone();
            }
        }
    }
}