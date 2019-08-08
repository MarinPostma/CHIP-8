
    pub fn emulate(&mut self, ram: &RAM) {
        let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;
        let inst = hi << 8 | lo;
        let nnn = inst & 0x0fff;
        let nn = inst & 0x00ff;
        let n = inst & 0x000f;
        let x = ( inst & 0x0f00 >> 8 ) as usize;
        let y = ( inst & 0x00f0 >> 4 ) as usize;

        //println!("op: {:04x}", inst);
        match ( hi & 0xf0 ) >> 4 {
            0x00 => {
                match lo {
                    // RET
                    0xee => self.pc = self.stack.pop().expect("Stack Underflow!"),
                    // CLS: Clear screen
                    0xe0 => unimplemented!(),
                    _ => self.pc += 2,
                }
            },
            //JMP to nnn
            0x01 => self.pc = nnn,
            // Call nnn
            0x02 => {
                self.stack.push(self.pc);
                self.pc = nnn;
            },
            //Skip equal: skip if Vx == nn
            0x03 => {
                if self.v[x] == nn {
                   self.pc += 4; 
                } else {
                    self.pc += 2;
                }
            },
            //Skip not equal: skip if Vx != nn
            0x04 => {

                if self.v[x] != nn {
                   self.pc += 4; 
                } else {
                    self.pc += 2;
                }
            },
            // Skip eq: skip if Vx == Vy
            0x05 => {
                if lo & 0x000f == 0 {
                    if self.v[x] == self.v[y as usize]  {
                        self.pc += 4; 
                    } else {
                        self.pc += 2;
                    }
                } else {
                    self.pc += 2
                }
            },
            // LOAD nn in Vx
            0x06 => {
                self.v[x] = nn;
                self.pc += 2;
            },
            // ADD nn to Vx
            0x07 => {
                self.v[x] += nn;
                self.pc += 2;
            },
            0x08 => {
                match lo & 0x000f {
                    //LOAD Vy in Vx
                    0x0 => self.v[x] = self.v[y],
                    0x1 => self.v[x] |= self.v[y],
                    0x2 => self.v[x] &= self.v[y],
                    0x3 => self.v[x] ^= self.v[y],
                    0x4 => {
                        self.v[x] += self.v[y];
                        if self.v[x] > 255 {
                            self.v[0xf] = 1;
                            self.v[x] &= 0x00ff;
                        } else {
                            self.v[0xf] = 0;
                        }
                    },
                    0x5 => {
                        if self.v[x] > self.v[y] {
                            self.v[0xf] = 1;
                        } else {
                            self.v[0xf] = 0;
                        }
                        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                    },
                    0x6 => {
                        self.v[0xf] = if self.v[x] & 0x0001 == 1 {1} else {0};
                        self.v[x] >>= 2;
                    },
                    0x7 => {
                        self.v[0xf] = if self.v[y] > self.v[x] {1} else {0};
                        self.v[y].wrapping_sub(self.v[x]);
                    },
                    0xe => {
                        self.v[0xf] = if self.v[x] & 0x8000 == 1 {1} else {0};
                        self.v[x] <<= 2;
                    },
                }
                self.pc += 2;
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
            },
            _ => self.pc += 2,
        }
    }














// Par exemple:
enum input {
    None, //pas d'input
    Str(String), // une string
    Nbr(u64), // un nombre
}

let foo = input::Str("je suis un user je rentre une string");

match foo {
    None => println!("L'utilisateur n'a rien rentre"),
    Str(s) => println!("L'utilisateur a rentre un string: {}", s),
    Nbr(n) => println!("L'utilisateur a rentre un nombre: ", n),
}

























