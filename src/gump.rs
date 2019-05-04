use super::structs::{Gump};

use std::{
    fs::{File},
    io::{SeekFrom, Seek}
};

pub fn lookup(lookup: u32) {
    let mut file = File::open("Gumpart.mul").
        expect("Unable to open file");

    file.seek(SeekFrom::Start(lookup as u64)).unwrap();

    let gump = Gump::from_reader(&file).unwrap();
    println!("0x{:02X}, 0x{:02X}", gump.width, gump.height);
}
