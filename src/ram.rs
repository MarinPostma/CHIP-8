
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

    pub fn mem_cpy(&mut self, src: &[u8], offset: usize) {
        let slice = &mut self.mem[offset..(offset + src.len())];
        slice.clone_from_slice(src);
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

    #[test]
    fn test_mem_cpy() {
        let mut ram = RAM::new();
        let data = [1, 2, 3, 4, 5];
        let offset = 0x200;
        assert_eq!(ram.read_byte(offset), 0);
        ram.mem_cpy(&data, offset);
        for i in offset..data.len() {
            assert_eq!(ram.read_byte(offset + i), data[i]);
        }
        assert_eq!(ram.read_byte(offset - 1), 0);
    }
}
