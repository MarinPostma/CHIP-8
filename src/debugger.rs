use crate::cpu::{Processor, CPU};
use std::io;

pub struct Debugger {
    pub cpu: CPU,
}

impl Debugger {
    pub fn new(cpu: CPU) -> Self {
        Debugger { cpu: cpu }
    }

    fn print_state(&self) {
        println!(
            "\x1b[2Jpc: 0x{:02x}\ni: 0x{:03x}\nregisters: {:?}\nkeys: {:?}\nop: 0x{:04x} {}",
            self.cpu.pc,
            self.cpu.i,
            self.cpu.v,
            self.cpu.key_press,
            (self.cpu.ram[self.cpu.pc] as u16) << 8 | self.cpu.ram[self.cpu.pc + 1] as u16,
            self.get_op()
        )
    }

    fn get_op(&self) -> String {
        let hi = self.cpu.ram[self.cpu.pc] as u16;
        let lo = self.cpu.ram[self.cpu.pc + 1] as u16;
        let inst = hi << 8 | lo;

        let nibs = (
            (inst & 0xf000) >> 12,
            (inst & 0x0f00) >> 8,
            (inst & 0x00f0) >> 4,
            (inst & 0x000f),
        );

        let nnn = (inst & 0x0fff) as usize;
        let nn = (inst & 0x00ff) as u8;
        let n = nibs.3 as u8;
        let x = nibs.1 as usize;
        let y = nibs.2 as usize;

        println!("inst: {:?}", nibs);
        match nibs {
            (0x00, 0x00, 0x0e, 0x0e) => format!("RET"),
            (0x00, 0x00, 0x0e, 0x00) => format!("CLS"),
            (0x01, _, _, _) => format!("JMP\t0x{:03x}", nnn),
            (0x02, _, _, _) => format!("CALL\t{:03x}", nnn),
            (0x03, _, _, _) => format!("SE\tV{:x}, {}", x, nn),
            (0x04, _, _, _) => format!("SNE\t V{:x}, {:02x}", x, nn),
            (0x05, _, _, 0x00) => format!("SE\tV{:x}, V{:x}", x, y),
            (0x06, _, _, _) => format!("LD\tV{:x}, {:02x}", x, nn),
            (0x07, _, _, _) => format!("ADD\tV{:x}, {:02x}", x, nn),
            (0x08, _, _, 0x00) => format!("LD\tV{:x}, V{:x}", x, y),
            (0x08, _, _, 0x01) => format!("OR\tV{:x}, V{:x}", x, y),
            (0x08, _, _, 0x02) => format!("AND\tV{:x}, V{:x}", x, y),
            (0x08, _, _, 0x03) => format!("XOR\tV{:x}, V{:x}", x, y),
            (0x08, _, _, 0x04) => format!("ADD\tV{:x}, V{:x}", x, y),
            (0x08, _, _, 0x05) => format!("SUB\tV{:x}, V{:x}", x, y),
            (0x08, _, _, 0x06) => format!("SHR\tV{:x}, {{V{:x}}}", x, y),
            (0x08, _, _, 0x07) => format!("SUBN\tV{:x}, V{:x}", x, y),
            (0x08, _, _, 0x0e) => format!("SHL\tV{:x}, {{V{:02x}}}", x, y),
            (0x09, _, _, 0x00) => format!("SNE\tV{:x}, V{:02x}", x, y),
            (0x0a, _, _, _) => format!("LD\t I, 0x{:03x}", nnn),
            (0x0b, _, _, _) => format!("JP\tV0, 0x{:03x}", nnn),
            (0x0c, _, _, _) => format!("RND\tV{:x}, 0x{:03x}", x, nn),
            (0x0d, _, _, _) => format!("DRW\tV{:x}, V{:x}, 0x{:02x}", x, y, n),
            (0x0e, _, 0x09, 0x0e) => format!("SKP\tV{:x}", x),
            (0x0e, _, 0x0a, 0x01) => format!("SKNP\tV{:x}", x),
            (0x0f, _, 0x00, 0x07) => format!("LD\tV{:x}, Dt", x),
            (0x0f, _, 0x00, 0x0a) => format!("LD\tV{:x}, K", x),
            (0x0f, _, 0x01, 0x05) => format!("LD\tDt, V{:x}", x),
            (0x0f, _, 0x01, 0x08) => format!("LD\tST, V{:x}", x),
            (0x0f, _, 0x01, 0x0e) => format!("ADD\tI, V{:x}", x),
            (0x0f, _, 0x02, 0x09) => format!("LD\tF, V{:x}", x),
            (0x0f, _, 0x03, 0x03) => format!("LD\tB, V{:x}", x),
            (0x0f, _, 0x05, 0x05) => format!("LD\t[I], V{:x}", x),
            (0x0f, _, 0x06, 0x05) => format!("LD\tV{:x}, [I]", x),
            _ => String::new(),
        }
    }
}

impl Processor for Debugger {
    fn load_rom(&mut self, rom: &[u8]) {
        self.cpu.load_rom(rom);
    }

    fn get_vram_buffer(&self, buffer: &mut [(u8, u8, u8)]) {
        self.cpu.get_vram_buffer(buffer)
    }

    fn set_key_press(&mut self, key: u8, is_down: bool) {
        self.cpu.set_key_press(key, is_down);
    }

    fn should_redraw(&self) -> bool {
        self.cpu.should_redraw()
    }

    fn drawn(&mut self) {
        self.cpu.drawn();
    }

    fn tick(&mut self) {
        self.print_state();
        self.cpu.tick();
    }
}

fn wait_command() -> bool {
    let mut input = String::new();
    print! {"> "};
    io::stdin().read_line(&mut input).unwrap();
    if input == "n" {
        true
    } else {
        false
    }
}
