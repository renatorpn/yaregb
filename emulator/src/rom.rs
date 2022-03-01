use std::fs;

pub struct Rom{
    data: Vec<u8>
}

impl Rom{

    pub fn new(file: &str) -> Rom{
        let contents = fs::read(file).expect("Could not read ROM file");

        Rom{
            data: contents
        }
    }

    pub fn get_byte(&self mut, addr: usize) -> Byte{
        self.data[addr]
    }
    
    pub fn get_cartridge_type(&self mut, addr: usize) -> Byte{
        self.data[0x0147]
    }
    
    pub fn length(&self) -> usize{
        self.data.len()
    }
    
    pub fn get_number_banks(&self) -> u16{
        match self.data[0x0148]{
            0x00 => 1,
            0x01 => 4,
            0x02 => 8,
            0x03 => 16,
            0x04 => 32,
            0x05 => 64,
            0x06 => 128,
            0x07 => 256,
            0x08 => 512,
            0x52 => 72,
            0x53 => 80,
            0x54 => 96,
            _ => 2,
        }
    }
}
