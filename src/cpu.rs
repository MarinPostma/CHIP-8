use rand::prelude::*;

const PGM_OFFSET: usize = 0x200;

// stack's not in the ram for convenience, as it is use
// to store adresses that are 16 bits
pub struct CPU {
    v: [u8; 16],
    i: usize,
    ram: [u8; 4096],
    stack: Vec<usize>,
    pc: usize,
    delay: u8,
    sound: u8,
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
        Self {
            v: [0; 16],
            i: 0,
            stack: Vec::with_capacity(16),
            ram: [0; 4096],
            pc: PGM_OFFSET,
            delay: 0,
            sound: 0,
        }
    }

    pub fn emulate(&mut self) {
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
        let n = (inst & 0x000f) as u8;
        let x = (inst & 0x0f00 >> 8) as usize;
        let y = (inst & 0x00f0 >> 4) as usize;

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
    }

    // RET
    // TODO: write test
    fn op_00ee(&mut self) -> PcJump {
        self.pc = self.stack.pop().expect("Stack Underflow!") as usize;
        PcJump::None
    }

    // CLS: Clear screen
    // TODO: write test
    fn op_00e0(&mut self) -> PcJump {
        unimplemented!("Op not implemented!");
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
        self.v[x] += nn;
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

    // TODO: write test
    fn op_8xy4(&mut self, x: usize, y: usize) -> PcJump {
        self.v[x] += self.v[y];
        if self.v[x] > 255 {
            self.v[0xf] = 1;
            self.v[x] &= 0x00ff;
        } else {
            self.v[0xf] = 0;
        }
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

    // TODO: write test
    fn op_dxyn(&mut self, x: usize, y: usize, n: u8) -> PcJump {
        unimplemented!("Op not implemented!");
    }

    // TODO: write test
    fn op_ex9e(&mut self, x: usize) -> PcJump {
        unimplemented!("Op not implemented!");
    }

    // TODO: write test
    fn op_exa1(&mut self, x: usize) -> PcJump {
        unimplemented!("Op not implemented!");
    }

    // TODO: write test
    fn op_fx07(&mut self, x: usize) -> PcJump {
        self.v[x] = self.delay;
        PcJump::Next
    }

    // TODO: write test
    fn op_fx0a(&mut self, x: usize) -> PcJump {
        unimplemented!("Op not implemented!");
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
        unimplemented!("Op not implemented!");
    }

    // TODO: write test
    fn op_fx33(&mut self, x: usize) -> PcJump {
        unimplemented!("Op not implemented!");
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
        unimplemented!("Op not implemented!");
    }

    // TODO: write test
    fn mem_cpy(&mut self, src: &[u8], offset: u16) {
        let offset = offset as usize;
        let slice = &mut self.ram[offset..(offset + src.len())];
        slice.clone_from_slice(src);
    }
}

#[cfg(test)]
mod tests {}
