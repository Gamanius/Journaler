#![windows_subsystem = "windows"]
pub mod ui;
pub mod journal;

use crate::{journal::{Journal, journal_helper::journal_add_new, journal_save::open_journal}, ui::window::WindowFrame};


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // args[0] is the executable path
    // args[1..] are additional arguments (like the file path)
    let journal_path = args.get(1).map(|s| std::path::PathBuf::from(s));

    // Load the journal if a path was provided
    let mut main_journal = if let Some(path) = journal_path {
        match open_journal(&path) {
            Ok(j) => j,
            Err(_) => Journal::default(),
        }
    } else {
        Journal::default()
    };

    journal_add_new(&mut main_journal);

    eframe::run_native(
        "Journal",
        options,
        Box::new(|cc| Ok(Box::new(WindowFrame::new(cc, main_journal)))),
    )
}
