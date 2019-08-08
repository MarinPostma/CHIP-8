pub mod cpu;
pub mod rom_mgr;

use std::env::args;

fn main() {
    let filename = args().into_iter().nth(1).expect("A ROM file should be provided as argument");
    let rom = rom_mgr::RomMgr::new(filename);
}
