use byteorder::{LittleEndian, ReadBytesExt};

use std::{
    fs::File,
    io::{self, Read}
};

#[derive(Debug)]
struct VerData {
    file_id: i32,
    block_id: i32,
    lookup: i32,
    length: i32,
    extra: i32
}

impl VerData {
    fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let file_id = rdr.read_i32::<LittleEndian>().unwrap();
        let block_id = rdr.read_i32::<LittleEndian>().unwrap();
        let lookup = rdr.read_i32::<LittleEndian>().unwrap();
        let length = rdr.read_i32::<LittleEndian>().unwrap();
        let extra = rdr.read_i32::<LittleEndian>().unwrap();

        Ok(VerData {
            file_id,
            block_id,
            lookup,
            length,
            extra
        })
    }
}

fn file_lookup(num: i32) -> &'static str {
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

pub fn verdata(file_name: String) {
    let mut buf: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
    let mut file = File::open(file_name)
        .expect("Unable to open file");

    file.read(&mut buf).unwrap();
    let patch_count = i32::from_le_bytes(buf);

    for n in 1..=patch_count {
        let verdata = VerData::from_reader(&file).unwrap();
        let identifier = file_lookup(verdata.file_id);

        println!("BLOCK #{} [ file_id: {}, block_id: {}, lookup: {}, length: {}, extra: {} ]",
                 n,
                 identifier,
                 verdata.block_id,
                 verdata.lookup,
                 verdata.length,
                 verdata.extra);
    }
}
