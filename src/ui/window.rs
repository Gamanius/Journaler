use egui::{RichText};

use crate::journal::{Journal, journal_editor::JournalEditor, journal_save::JournalSave, journal_sidebar::JournalSidebar};

#[derive(Default)]
pub struct WindowFrame {
    editor: JournalEditor,
    sidbar: JournalSidebar,
    save: JournalSave,
    journal: Journal,

}

impl WindowFrame {
    pub fn new(cc: &eframe::CreationContext<'_>, j: Journal) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        Self {
            journal: j,
            sidbar: JournalSidebar { selected_day: chrono::Local::now().date_naive() },
            ..Default::default()
        }
    }
}

impl eframe::App for WindowFrame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if let Some(title) = self.journal.path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|f| f.to_str())
            .map(|s| format!("Journal: {}", s))
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(title));
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title("Journal".to_string()));
        }


        egui::TopBottomPanel::top("my_top_panel").show(ctx, |ui| {
            self.save.update(ui, &mut self.journal);
        });

        egui::SidePanel::left("my_left_panel")
        .resizable(true)        // allow resizing
        .show(ctx, |ui| {
           self.sidbar.update(ui, &mut self.journal);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint_after(std::time::Duration::from_secs(1));
            
            let selec_day = self.journal.days.get_mut(&self.sidbar.selected_day);
            match selec_day {
                Some(t) => self.editor.update(ui, t, self.sidbar.selected_day),
                _ => {ui.label(RichText::new("Nichts ausgewählt"));} ,
            }
        });
    }
}