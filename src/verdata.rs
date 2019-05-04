use std::{
    fs::{File},
    io::{Read}
};
use super::structs::{VerData};
use super::utils::print_json;

fn file_map(num: i32) -> &'static str {
    match num {
       0 => "Map0.mul",
       1 => "Staidx0.mul",
       2 => "Statics0.mul",
       3 => "Artidx.mul",
       4 => "Art.mul",
       5 => "Anim.idx",
       6 => "anim.mul",
       7 => "SoundIdx.mul",
       8 => "Sound.mul",
       9 => "TexIdx.mul",
       10 => "TexMaps.mul",
       11 => "GumpIdx.mul",
       12 => "GumpArt.mul",
       13 => "Multi.idx",
       14 => "Multi.mul",
       15 => "Skills.idx",
       16 => "Skills.mul",
       30 => "Tiledata.mul",
       31 => "Animdata.mul",
       _ => "Unknown",
    }
}

pub fn to_json(file_name: String) {
    let mut buf: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
    let mut file = File::open(file_name)
        .expect("Unable to open file");

    file.read(&mut buf).unwrap();
    let patch_count = i32::from_le_bytes(buf);
    let mut entries: Vec<VerData> = Vec::new();

    for _ in 1..=patch_count {
        let verdata = VerData::from_reader(&file).unwrap();
        let identifier = file_map(verdata.file_id);

        if identifier != "Unknown" {
            entries.push(verdata);
        }
    }

    print_json(entries, &serde_json::to_string);
}
