use crate::journal::{Journal, JournalDay};

pub fn journal_add_new(j: &mut Journal) -> bool {
    let time = chrono::Local::now();
    let date = time.date_naive();

    let res = j.days.get(&date);

    let day = JournalDay {
        entries: vec![],
    };

    match res {
        Some(_) => return false,
        None => {
            j.days.insert(date, day);
            return true;
        }
    }
}