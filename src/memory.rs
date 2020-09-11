#[derive(Debug, Default)]
pub struct Memory {
    ram: Box<[u8]>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            ram: vec![0; 4 * 1024].into_boxed_slice(),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn read_doublebyte(&self, address: u16) -> u16 {
        ((self.ram[address as usize] as u16) << 8) | self.ram[address as usize + 1] as u16
    }

    pub fn write_chunk(&mut self, address: u16, chunk: Box<[u8]>) {
        let mut count = 0;
        for byte in chunk.iter() {
            self.write_byte(address + count, *byte);
            count += 1;
        }
    }

    pub fn read_chunk(&mut self, address: u16, size: usize) -> Vec<u8> {
        let mut chunk = vec![0; size];

        for i in 0..size {
            chunk[i] = self.ram[(address as usize) + size];
            println!("{:#010b}", chunk[i]);
        }

        chunk
    }
}
