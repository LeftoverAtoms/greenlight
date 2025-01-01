use binrw::{BinRead, BinReaderExt};
use std::{fs::{self}, io::Cursor};

use crate::{app, App};

#[derive(Default)]
pub struct IPak {
    pub header: IPakHeader,
    pub index_section: IPakSection,
    pub data_section: IPakSection,
}

#[derive(BinRead, Default)]
pub struct IPakHeader {
    pub magic: u32,
    pub version: u32,
    pub size: u32,
    pub sections: u32,
}

#[derive(BinRead, Default)]
pub struct IPakSection {
    pub kind: u32,
    pub offset: u32,
    pub size: u32,
    pub count: u32,
}

#[derive(BinRead, Default)]
pub struct IPakEntry {
    pub key: u64,
    pub offset: u32,
    pub size: u32,
}

impl IPak {
    pub fn parse(app: &mut App, file_path: &str) {
        let mut ipak: IPak = IPak::default();

        let Ok(data) = fs::read(file_path) else { return; };

        let mut reader = Cursor::new(data);

        match reader.read_le::<IPakHeader>() {
            Ok(header) => ipak.header = header,
            Err(e) => (),
        };

        if ipak.header.magic != 0x4950414b {
            return;
        }

        for _ in 0..ipak.header.sections {
            let Ok(segment) = reader.read_le::<IPakSection>() else { continue; };

            match segment.kind {
                1 => ipak.index_section = segment,
                2 => ipak.data_section = segment,
                _ => {},
            }
        };

        reader.set_position(ipak.index_section.offset.into());

        for _ in 0..ipak.index_section.count {
            let Ok(entry) = reader.read_le::<IPakEntry>() else { continue; };
            app.assets.insert(app::Asset {
                name: format!("_{:x}", entry.key),
                selected: false,
            });
        }
    }
}
