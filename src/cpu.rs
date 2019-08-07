use crate::ram::RAM;

const INIT_PGM: u16 = 0x200;
const INIT_SP u16 = 0xfa0;

pub struct CPU {
    v: [u8; 16],
    i: u16,
    sp: u16,
    pc: u16,
    delay: u8,
    sound: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            sp: 0xfa0,
            pc: INIT_PGM,
            delay: 0,
            sound: 0,
        }
    }

    pub fn emulate(&mut self, ram: &RAM) {
        let hi = ram.read_byte(self.pc as usize) as u16;
        let lo = ram.read_byte(( self.pc + 1 ) as usize) as u16;
        let inst = hi << 8  | lo;
        match inst {
            0x00 => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x01 => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x02 => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x03 => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x04 => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x05 => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x06 => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x07 => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x08 => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x09 => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x0a => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x0b => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x0c => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x0d => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x0e => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            0x0f => {
                unimplemented!("Instruction {:02x} not yet implemented", inst);
            }
            _ => unimplemented!("Instruction {:02x} not yet implemented", inst),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CPU;

    #[test]
    fn test_new() {
        let cpu = CPU::new();

        assert_eq!(cpu.delay, 0);
        assert_eq!(cpu.pc, 0x200);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.i, 0);
        assert_eq!(cpu.sound, 0);
        assert_eq!(cpu.v, [0; 16]);
    }
}
