use crate::cpu::CPU;
use crate::ram::RAM;

struct Chip8 {
    cpu: CPU,
    ram: RAM,
}

impl Chip8 {

    pub fn new() -> Self {
        cpu: CPU::new(),
        ram: RAM::new(),
    }
}
