use byteorder::{LittleEndian, ReadBytesExt};
use super::structs::{EntryID, Gump};
use super::utils::print_json;

use std::{
    fs::{self, File},
    io::{SeekFrom, Seek}
};

pub fn read_binary(data_file: &str, index_file: &str, entry_id: usize) {
    let entries = load_entries_as_vec(index_file);
    let entry = &entries[entry_id];
    let mut file = File::open(data_file).unwrap();
    let mut bytes: Vec<i32> = Vec::new();
    file.seek(SeekFrom::Start(entry.lookup as u64)).unwrap();

    for i in 0..entry.length {
        let byte = file.read_i32::<LittleEndian>().unwrap();
        bytes.push(byte)
    }

    println!("{:X?}", bytes);
}

pub fn to_json(file_name: &str) {
    let entries = load_entries_as_vec(file_name);
    print_json(entries, &serde_json::to_string);
}

fn load_entries_as_vec(file_name: &str) -> Vec<EntryID> {
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

    entries
}
