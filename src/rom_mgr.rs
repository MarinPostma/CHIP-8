use std::fs::File;
use std::io::Read;

struct rom_mgr {
    name: String,
    bin: Vec<u8>,
}

impl rom_mgr {
    pub fn new(filename: String) -> Self {
        let mut file = File::open(&filename).expect("error while opening the file.");
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("The ROM could not be read.");
        Self {
            name: filename.split("/").last().unwrap().to_string(),
            bin: data,
        }
    }
}