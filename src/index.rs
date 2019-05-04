use super::structs::{EntryID, Gump};
use super::utils::print_json;

use std::{
    fs::{self, File},
    io::{SeekFrom, Seek}
};

pub fn to_json(file_name: String) {
    let file = File::open(&file_name).unwrap();
    let file_size = fs::metadata(&file_name).unwrap().len();
    let num_entries = file_size / 12;
    let mut entries: Vec<EntryID> = Vec::new();

    for _ in 1..=num_entries {
        let entry = EntryID::from_reader(&file).unwrap();
        if entry.lookup != -1 {
            entries.push(entry);
        }
    }

    print_json(entries, &serde_json::to_string);
}
