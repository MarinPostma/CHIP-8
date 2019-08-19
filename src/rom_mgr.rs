use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
pub struct RomMgr {
    name: String,
    pub bin: Vec<u8>,
}

impl RomMgr {
    pub fn new(filename: &str) -> Self {
        let mut file = File::open(&filename).expect("error while opening the file.");
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("The ROM could not be read.");
        Self {
            name: filename.split("/").last().unwrap().to_string(),
            bin: data,
        }
    }
}
