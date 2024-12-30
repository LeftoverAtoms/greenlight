use binrw::{BinRead, BinReaderExt};
use std::{fs::{self}, io::Cursor};

#[derive(BinRead, Default)]
pub struct IPAK {
    pub magic: u32,
    pub version: u32,
    pub size: u32,
    pub segment_count: u32,
}

impl IPAK {
    pub fn parse(file_path: &str) -> IPAK {
        let file = fs::read(file_path).unwrap();
        let mut reader = Cursor::new(file);

        // IPAK will be read as KAPI in little endian order.
        reader.read_be::<IPAK>().unwrap()
    }
}
