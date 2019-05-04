use serde::{Serialize};
use std::{
    io::{self, Read}
};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Serialize)]
pub struct EntryID {
    pub lookup: i32,
    pub length: i32,
    pub extra: i32
}

#[derive(Debug, Serialize)]
pub struct VerData {
    pub file_id: i32,
    pub block_id: i32,
    pub entry: EntryID
}

#[derive(Debug)]
pub struct Gump {
    pub width: i16,
    pub height: i16
}

impl Gump {
    pub fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let width = rdr.read_i16::<LittleEndian>().unwrap();
        let height = rdr.read_i16::<LittleEndian>().unwrap();

        Ok(Gump {
            width,
            height
        })
    }
}

impl EntryID {
    pub fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let lookup = rdr.read_i32::<LittleEndian>().unwrap();
        let length = rdr.read_i32::<LittleEndian>().unwrap();
        let extra = rdr.read_i32::<LittleEndian>().unwrap();

        Ok(EntryID {
            lookup,
            length,
            extra
        })
    }
}

impl VerData {
    pub fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let file_id = rdr.read_i32::<LittleEndian>().unwrap();
        let block_id = rdr.read_i32::<LittleEndian>().unwrap();
        let entry = EntryID::from_reader(rdr).unwrap();

        Ok(VerData {
            file_id,
            block_id,
            entry
        })
    }
}

