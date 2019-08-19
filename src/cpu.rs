use crate::rom_mgr::RomMgr;
use rand::prelude::*;
use v_display::sdl2::keyboard::Keycode;

const PGM_OFFSET: usize = 0x200;

pub trait Tick {
    fn tick(&mut self);
}

pub struct CPU {
    pub v: [u8; 16],
    pub i: usize,
    pub ram: [u8; 4096],
    pub vram: [(u8, u8, u8); 64 * 32],
    pub stack: Vec<usize>,
    pub pc: usize,
    pub delay: u8,
    pub sound: u8,
    pub key_press: [bool; 16],
    pub draw: bool,
}

enum PcJump {
    None,
    Next,
    Skip,
}

impl PcJump {
    pub fn to_int(self) -> usize {
        match self {
            PcJump::None => 0,
            PcJump::Next => 2,
            PcJump::Skip => 4,
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu = Self {
            v: [0; 16],
            i: 0,
            stack: Vec::with_capacity(16),
            ram: [0; 4096],
            vram: [(0, 0, 0); 64 * 32],
            pc: PGM_OFFSET,
            delay: 0,
            sound: 0,
            key_press: [false; 16],
            draw: false,
        };
        cpu.mem_cpy(&include!("chars.in"), 0);
        cpu
    }

    // TODO: write test
    pub fn load_rom(&mut self, rom: RomMgr) {
        self.mem_cpy(&rom.bin, PGM_OFFSET);
    }

    pub fn set_key_down(&mut self, key: Keycode, pos: bool) {
        use Keycode::*;
        match key {
            Num1 => self.key_press[1] = pos,
            Num2 => self.key_press[2] = pos,
            Num3 => self.key_press[3] = pos,
            Num4 => self.key_press[0xc] = pos,
            Q => self.key_press[4] = pos,
            W => self.key_press[5] = pos,
            E => self.key_press[6] = pos,
            R => self.key_press[0xd] = pos,
            A => self.key_press[7] = pos,
            S => self.key_press[8] = pos,
            D => self.key_press[9] = pos,
            F => self.key_press[0xe] = pos,
            Z => self.key_press[0xa] = pos,
            X => self.key_press[0] = pos,
            C => self.key_press[0xb] = pos,
            V => self.key_press[0xf] = pos,
            _ => (),
        }
    }
}

impl Tick for CPU {
    // TODO: write test
    fn tick(&mut self) {
        let hi = self.ram[self.pc] as u16;
        let lo = self.ram[self.pc + 1] as u16;
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

        self.pc += match nibs {
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xnn(x, nn),
            (0x04, _, _, _) => self.op_4xnn(x, nn),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xnn(x, nn),
            (0x07, _, _, _) => self.op_7xnn(x, nn),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8xy6(x, y),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8xye(x, y),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0a, _, _, _) => self.op_annn(nnn),
            (0x0b, _, _, _) => self.op_bnnn(nnn),
            (0x0c, _, _, _) => self.op_cxnn(x, nn),
            (0x0d, _, _, _) => self.op_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_fx65(x),
            _ => PcJump::Next,
        }
        .to_int();
        //reinit keypress
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            self.sound -= 1;
        }
    }
}

impl CPU {
    // RET
    // TODO: write test
    fn op_00ee(&mut self) -> PcJump {
        self.pc = self.stack.pop().expect("Stack Underflow!") as usize;
        PcJump::Next
    }

    // CLS: Clear screen
    fn op_00e0(&mut self) -> PcJump {
        self.vram.iter_mut().for_each(|x| *x = (0, 0, 0));
        PcJump::Next
    }

    //JMP to nnn
    // TODO: write test
    fn op_1nnn(&mut self, nnn: usize) -> PcJump {
        self.pc = nnn as usize;
        PcJump::None
    }

    // CALL nnn
    // TODO: write test
    fn op_2nnn(&mut self, nnn: usize) -> PcJump {
        self.stack.push(self.pc);
        self.pc = nnn as usize;
        PcJump::None
    }

    //SKIP.EP: skip if Vx == nn
    // TODO: write test
    fn op_3xnn(&mut self, x: usize, nn: u8) -> PcJump {
        if self.v[x] == nn {
            PcJump::Skip
        } else {
            PcJump::Next
        }
    }

    //SKIP.NE: skip if Vx != nn
    // TODO: write test
    fn op_4xnn(&mut self, x: usize, nn: u8) -> PcJump {
        if self.v[x] != nn {
            PcJump::Skip
        } else {
            PcJump::Next
        }
    }

    //SKIP.EP: skip if Vx == Vy
    // TODO: write test
    fn op_5xy0(&mut self, x: usize, y: usize) -> PcJump {
        if self.v[x] == self.v[y as usize] {
            PcJump::Skip
        } else {
            PcJump::Next
        }
    }

    // LOAD nn in Vx
    // TODO: write test
    fn op_6xnn(&mut self, x: usize, nn: u8) -> PcJump {
        self.v[x] = nn;
        PcJump::Next
    }

    // ADD nn to Vx
    // TODO: write test
    fn op_7xnn(&mut self, x: usize, nn: u8) -> PcJump {
        let vx = self.v[x] as u16;
        let nn = nn as u16;
        self.v[x] = (nn + vx) as u8;
        PcJump::Next
    }

    //LOAD Vy in Vx
    // TODO: write test
    fn op_8xy0(&mut self, x: usize, y: usize) -> PcJump {
        self.v[x] = self.v[y];
        PcJump::Next
    }

    // TODO: write test
    fn op_8xy1(&mut self, x: usize, y: usize) -> PcJump {
        self.v[x] |= self.v[y];
        PcJump::Next
    }

    // TODO: write test
    fn op_8xy2(&mut self, x: usize, y: usize) -> PcJump {
        self.v[x] &= self.v[y];
        PcJump::Next
    }

    // TODO: write test
    fn op_8xy3(&mut self, x: usize, y: usize) -> PcJump {
        self.v[x] ^= self.v[y];
        PcJump::Next
    }

    fn op_8xy4(&mut self, x: usize, y: usize) -> PcJump {
        self.v[0xf] = 0;
        self.v[x] = match self.v[x].checked_add(self.v[y]) {
            Some(val) => val,
            None => {
                self.v[0xf] = 1;
                let vx = self.v[x] as u16;
                let vy = self.v[y] as u16;
                (vx + vy) as u8
            }
        };
        PcJump::Next
    }

    // TODO: write test
    fn op_8xy5(&mut self, x: usize, y: usize) -> PcJump {
        if self.v[x] > self.v[y] {
            self.v[0xf] = 1;
        } else {
            self.v[0xf] = 0;
        }
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        PcJump::Next
    }

    // TODO: write test
    fn op_8xy6(&mut self, x: usize, _y: usize) -> PcJump {
        self.v[0xf] = if self.v[x] & 0x01 == 1 { 1 } else { 0 };
        self.v[x] >>= 2;
        PcJump::Next
    }

    // TODO: write test
    fn op_8xy7(&mut self, x: usize, y: usize) -> PcJump {
        self.v[0xf] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        PcJump::Next
    }

    // TODO: write test
    fn op_8xye(&mut self, x: usize, _y: usize) -> PcJump {
        self.v[0xf] = if self.v[x] & 0x80 == 1 { 1 } else { 0 };
        self.v[x] <<= 2;
        PcJump::Next
    }

    // TODO: write test
    fn op_9xy0(&mut self, x: usize, y: usize) -> PcJump {
        if self.v[x] != self.v[y] {
            PcJump::Skip
        } else {
            PcJump::Next
        }
    }

    // TODO: write test
    fn op_annn(&mut self, nnn: usize) -> PcJump {
        self.i = nnn;
        PcJump::Next
    }

    // TODO: write test
    fn op_bnnn(&mut self, nnn: usize) -> PcJump {
        self.pc = (self.v[0] as usize + nnn) as usize;
        PcJump::Next
    }

    // TODO: write test
    fn op_cxnn(&mut self, x: usize, nn: u8) -> PcJump {
        self.v[x] = random::<u8>() & nn;
        PcJump::Next
    }

    fn op_dxyn(&mut self, x: usize, y: usize, n: u8) -> PcJump {
        self.draw = true;
        for offset_y in 0..n as usize {
            let byte = self.ram[self.i + offset_y];
            for offset_x in 0..8 {
                let pixel = self.is_set_pix_at(
                    (self.v[x] as usize + offset_x) % 64,
                    (self.v[y] as usize + offset_y) % 32,
                );
                let new_pixel = byte & (0x80 >> offset_x) != 0;
                if pixel && new_pixel {
                    self.v[0xf] = 1;
                }
                self.set_pix_at(
                    (self.v[x] as usize + offset_x) % 64,
                    (self.v[y] as usize + offset_y) % 32,
                    pixel ^ new_pixel,
                );
            }
        }
        PcJump::Next
    }

    // TODO: write test
    fn op_ex9e(&mut self, x: usize) -> PcJump {
        if self.key_press[self.v[x] as usize] {
            PcJump::Skip
        } else {
            PcJump::Next
        }
    }

    // TODO: write test
    fn op_exa1(&mut self, x: usize) -> PcJump {
        if !self.key_press[self.v[x] as usize] {
            println!("skip");
            PcJump::Skip
        } else {
            println!("next");
            PcJump::Next
        }
    }

    // TODO: write test
    fn op_fx07(&mut self, x: usize) -> PcJump {
        self.v[x] = self.delay;
        PcJump::Next
    }

    // TODO: write test
    fn op_fx0a(&mut self, x: usize) -> PcJump {
        if self.key_press.iter().any(|k| *k) {
            self.v[x] = self
                .key_press
                .iter()
                .enumerate()
                .find(|(_, x)| **x)
                .unwrap()
                .0 as u8;
            PcJump::Next
        } else {
            PcJump::None
        }
    }

    // TODO: write test
    fn op_fx15(&mut self, x: usize) -> PcJump {
        self.delay = self.v[x] as u8;
        PcJump::Next
    }

    // TODO: write test
    fn op_fx18(&mut self, x: usize) -> PcJump {
        self.sound = self.v[x] as u8;
        PcJump::Next
    }

    // TODO: write test
    fn op_fx1e(&mut self, x: usize) -> PcJump {
        self.i += self.v[x] as usize;
        PcJump::Next
    }

    // TODO: write test
    fn op_fx29(&mut self, x: usize) -> PcJump {
        self.i = x * 5;
        PcJump::Next
    }

    fn op_fx33(&mut self, x: usize) -> PcJump {
        let i = self.i;
        self.ram[i] = self.v[x] / 100;
        self.ram[i + 1] = (self.v[x] % 100) / 10;
        self.ram[i + 2] = self.v[x] % 10;
        PcJump::Next
    }

    // TODO: write test
    fn op_fx55(&mut self, x: usize) -> PcJump {
        for n in 0..=x {
            self.ram[self.i as usize + n] = self.v[n] as u8;
        }
        PcJump::Next
    }

    // TODO: write test
    fn op_fx65(&mut self, x: usize) -> PcJump {
        for i in 0..=x {
            self.v[i] = self.ram[self.i + i];
        }
        PcJump::Next
    }

    // TODO: write test
    fn mem_cpy(&mut self, src: &[u8], offset: usize) {
        let slice = &mut self.ram[offset..(offset + src.len())];
        slice.clone_from_slice(src);
    }

    fn is_set_pix_at(&self, x: usize, y: usize) -> bool {
        self.vram[y * 64 + x] == (255, 255, 255)
    }

    fn set_pix_at(&mut self, x: usize, y: usize, on: bool) {
        self.vram[y * 64 + x] = if on { (255, 255, 255) } else { (0, 0, 0) };
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_init_cpu() {
        let cpu = CPU::new();
        let chars: &[u8] = &include!("chars.in");
        assert_eq!(&cpu.ram[0..chars.len()], chars);
    }

    #[test]
    fn test_op_8xy4() {
        let mut cpu = CPU::new();
        assert_eq!(cpu.v[0xf], 0);

        //check addition with no overflow
        cpu.v[1] = 3;
        cpu.v[2] = 2;
        cpu.op_8xy4(1, 2);
        assert_eq!(cpu.v[0xf], 0);
        assert_eq!(cpu.v[1], 5);

        //check addition with overflow
        cpu.v[1] = 255;
        cpu.v[2] = 255;
        cpu.op_8xy4(1, 2);
        assert_eq!(cpu.v[0xf], 1);
        assert_eq!(cpu.v[1], 0xfe);

        //check Vf is reset on next addition :
        cpu.v[1] = 3;
        cpu.v[2] = 2;
        cpu.op_8xy4(1, 2);
        assert_eq!(cpu.v[0xf], 0);
        assert_eq!(cpu.v[1], 5);
    }

    #[test]
    fn test_op_fx33() {
        let mut cpu = CPU::new();
        // test number > 100
        cpu.v[1] = 253;
        cpu.i = 0x600;
        cpu.op_fx33(1);
        assert_eq!(cpu.ram[cpu.i], 2);
        assert_eq!(cpu.ram[cpu.i + 1], 5);
        assert_eq!(cpu.ram[cpu.i + 2], 3);

        //test 10 < number < 100
        cpu.v[1] = 53;
        cpu.i = 0x600;
        cpu.op_fx33(1);
        assert_eq!(cpu.ram[cpu.i], 0);
        assert_eq!(cpu.ram[cpu.i + 1], 5);
        assert_eq!(cpu.ram[cpu.i + 2], 3);

        //test 0 < number < 10
        cpu.v[1] = 3;
        cpu.i = 0x600;
        cpu.op_fx33(1);
        assert_eq!(cpu.ram[cpu.i], 0);
        assert_eq!(cpu.ram[cpu.i + 1], 0);
        assert_eq!(cpu.ram[cpu.i + 2], 3);
        //
        //test number = 0
        cpu.v[1] = 0;
        cpu.i = 0x600;
        cpu.op_fx33(1);
        assert_eq!(cpu.ram[cpu.i], 0);
        assert_eq!(cpu.ram[cpu.i + 1], 0);
        assert_eq!(cpu.ram[cpu.i + 2], 0);
    }

    #[test]
    fn op_dxyn() {
        let mut cpu = CPU::new();

        cpu.ram[0x200] = 0xd0;
        cpu.ram[0x201] = 0x02;
        cpu.v[0] = 1;
        cpu.v[1] = 1;
        cpu.ram[0x202] = 0xd0;
        cpu.ram[0x203] = 0x02;
        cpu.ram[0x204] = 0x12;
        cpu.ram[0x205] = 0x00;
        cpu.ram[0x600] = 0xd0;
        cpu.ram[0x601] = 0xd0;
        assert_eq!(cpu.v[0xf], 0);
        cpu.tick();
        cpu.tick();
        assert_eq!(cpu.v[0xf], 1);
    }

    #[test]
    fn test_op_00e0() {
        let mut cpu = CPU::new();

        cpu.vram[8] = (2, 64, 34);
        let expected: &[(u8, u8, u8)] = &[(0, 0, 0); 64 * 32];
        cpu.op_00e0();
        for i in 0..2048 {
            assert_eq!(cpu.vram[i], expected[i]);
        }
    }
}
