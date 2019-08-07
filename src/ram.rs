
pub struct RAM {
    mem: [u8; 4096],
}

impl RAM {

    pub fn new() -> Self {
        RAM {
            mem: [0; 4096]
        }
    }

    pub fn read_byte(&self, position: usize) -> u8 {
        self.mem[position]
    }

    pub fn write_byte(&mut self, position: usize, value: u8) {
        self.mem[position] = value
    }
}

#[cfg(test)]
mod tests {
    use super::RAM;

    #[test]
    fn test_new() {
        assert_eq!(RAM::new().mem.len(), 4096);
    }

    #[test]
    fn test_read_write() {
        let mut ram = RAM::new();
        assert_eq!(ram.read_byte(5), 0);
        ram.write_byte(42, 0x1f);
        assert_eq!(ram.read_byte(42), 0x1f);
    }
}
