//maybe I should implement an object to extend to avoid code repetition, look at MbcType impls

use std::cmp;
use std::fmt;

use crate::rom::*;

pub enum MbcType {
    MBC1,
    MBC2,
    MBC3,
    MBC5,
}

pub trait Mbc{
    fn get_mbc_type(&self) -> MbcType;
    fn read_rom(&self, addr: Word) -> Byte;
    fn read_ram(&self, addr: Word) -> Byte;
    fn write_ram(&mut self, addr: Word, data: Byte);
    fn handle_bank(&mut self, addr: Word, data: Byte);
    fn get_ext_ram(&self) -> &[Byte];
    fn load_ext_ram(&mut self, buffer = Vec<Byte>);
}

pub struct Mbc1{
    memory: Vec<Byte>,
    rom_bank: usize,
    ram_bank: usize,
    ext_ram: [Byte; MAX_RAM_BANKS * RAM_BANK_SIZE],
    enable_ram: bool,
    ram_banks: u8,
    banking_mode: BankingMode,
}

pub struct Mbc2{
    memory: Vec<Byte>,
    rom_bank: usize,
    ext_ram: [Byte; 0x200],
    enable_ram: bool,
}

pub struct Mbc3{
    memory: Vec<Byte>,
    rom_bank: usize,
    ram_bank_or_rtc: usize,
    ext_ram: [Byte; MAX_RAM_BANKS * RAM_BANK_SIZE],
    enable_ram_rtc: bool,
    number_of_rom_banks: u8,

    //Real Time Clock if needed
    rtc_seconds: Byte,
    rtc_minutes: Byte,
    rtc_hours: Byte,
    rtc_dl: Byte,
    rtc_dh
}

pub struct Mbc5{
    memory: Vec<Byte>,
    rom_bank: usize,
    ram_bank: usize,
    ext_ram: [Byte; 16 * RAM_BANK_SIZE],
    enable_ram: bool,
    ram_banks: u8,
}

pub fn get_mbc(rom: &Rom) -> Option<Box<dyn Mbc>>{
    let rom_mode = rom.get_cartridge_type();
    match rom_mode {
        0x01 | 0x02 | 0x03 => Some(Box::new(Mbc1::new(rom))),
        0x05 | 0x06 => Some(Box::new(Mbc2::new(rom))),
        0x0F | 0x10 | 0x11 | 0x12 | 0x13 => Some(Box::new(Mbc3::new(rom))),
        0x19 | 0x1A | 0x1B | 0x1C | 0x1D | 0x1E => Some(Box::new(Mbc5::new(rom))),
        _ => None
    }
}

impl Mbc1{

    pub fn new(rom: &Rom) -> Mbc1{
        let mut memory = Vec::new();
        for i in 0..rom.length(){
            memory.push(rom.get_byte)(i));
        }

        Mbc1 {
            memory: memory,
            rom_bank: 1,
            ram_bank: 0,
            ext_ram: [0; MAX_RAM_BANKS * RAM_BANK_SIZE],
            enable_ram: false,
            number_of_rom_banks: rom.get_number_banks() as u8,
            banking_mode: BankingMode::ROM,
        }
    }
}

impl Mbc for Mbc1{
    fn get_mbc_type(&self) -> MbcType{
        MbcType::MBC1
    }

    fn read_rom(&self, addr: Word) -> Byte{
        let destination_addr = (addr as usize) + (self.rom_bank * 0x4000);
        self.memory[destination_addr]
    }

    fn read_ram(&self, addr: Word){
        self.ext_ram[(addr as usize) + (self.ram_bank * RAM_BANK_SIZE) as usize]
    }

    fn write_ram(&mut self, addr: Word, data: Byte){
        self.ext_ram[(addr as usize) + (self.ram_bank * RAM_BANK_SIZE)] = data;
    }

    fn handle_bank(&mut self, addr: Word, data: Byte){
        match addr{
            0x0000..=0x1FFF => if (data & 0x0F) == 0xA (self.enable_ram = true) else (self.enable_ram = false)},
            0x2000..=0x3FFF => {
                let new_rom_bank + data & 0x1F;
                self.rom_bank = (self.rom_bank & 0b11100000) | (new_rom_bank as usize);
                
                if self.rom_bank == 0 {
                    self.rom_bank += 1;
                }

                if self.rom_bank > self.number_of_rom_banks as usize{
                    println!("TO DO ADD MANY BANKS")
                }
            },
            0x4000..=0x5FFF => {
                match self.banking_mode{
                    BankingMode::RAM => self.ram_bank = (data & 0x03) as usize,
                    BankingMode::ROM => {
                        let new_rom_bank = data & 0xE0;

                        self.rom_bank = (new_rom_bank | ((self.rom_bank as u8) & 0b00011111)) as usize;
                        if self.rom_bank == 0{
                            self.rom_bank += 1;
                        }

                        if self.rom_bank > self.number_of_rom_banks as usize{
                            println!("TO DO ADD MANY BANKS")
                        }
                    }
                }
            },
            0x6000..=0x7FFF => {
                self.banking_mode = match bit_set(&data, 0){
                    true => BankingMode::RAM,
                    false => BankingMode::ROM,
                };
            },
            _ => println("INVALID ADDR {}", addr)
        };
    }

    fn get_ext_ram(&self) -> &[Byte]{
        &self.ext_ram
    }

    fn load_ext_ram(mut &self, buffer: Vec<Byte>){
        let ram_len = self.ext_ram.len();
        for i in 0..cmp::min(ram_len, buffer.len()){
            self.ext_ram[i] = buffer[i];
        }
    }

}

impl Mbc2{
    pub fn new(rom: &Rom) -> Mbc2{
        let mut memory = Vec::new();
        for i in 0..rom.length(){
            memory.push(rom.get_byte(i));
        }

        Mbc2{
            memory: memory,
            rom_bank: 1,
            ext_ram: [0; 0x200],
            enable_ram: false,
        }
    }
}

impl Mbc for Mbc2{
    fn get_mbc_type(&self) -> MbcType{
        MbcType::MBC2
    }

    fn read_ram(&self, addr: Word) -> Byte{
        let dest_addr = (addr as usize) % 0x200;
        self.ext_ram[dest_addr]
    }

    fn read_rom(&self, addr: Word) -> Byte{
        let dest_addr = (addr as usize) + (self.rom_bank * 0x4000);
        self.memory[dest_addr]
    }

    fn write_ram(&mut self, addr: Word, data: Byte){
        if self.enable_ram{
            let dest_addr = (addr as usize) % 0x200;
            self.ext_ram[dest_addr] = data & 0xF;
        }
    }

    fn handle_bank(&mut self, addr: Word, data: Byte){
        if addr >= 0x0000 && addr <= 0x4000 {
            let high_byte = (addr >> 8) as Byte;
        

        match is_bit_set{
            true =>{
                self.rom_bank = (data & 0xF) as usize;
                if self.rom_bank == 0{
                    self.rom_bank = 1;
                }
            },
            false =>{
                self.enable_ram == 0xA;
            }
        };
    }else{
        println!("INVALID ADDR FOR MBC2 - {:04X}", addr);
    }}

    fn get_ext_ram(&self) -> &[Byte]{
        self.ext_ram;
    }
    
    fn load_ext_ram(&mut self, buffer: Vec<Byte>){
        let ram_len = self.ext_ram.length();
        for i in 0..cmp::min(ram_len, buffer.len()){
            memory.push(rom.get_byte(i));
        }
    }

    }

}

impl Mbc3{
    pub fn new(rom: &Rom) -> Mbc3{
        let mut memory = Vec::new();
        for i in 0..rom.length(){
            memory.push(rom.get_byte([)i));
        }
    }

    Mbc3{
        memory: memory,
        rom_bank: 1,
        rom_bank_rtc: 0,
        ext_ram: [0; MAX_RAM_BANKS * RAM_BANK_SIZE],
        enable_ram_rtc: false,
        number_of_rom_banks: rom.get_number_banks() as u8,
        rtc_seconds: 0,
        rtc_minutes: 0,
        rtc_hours: 0,
        rtc_dh: 0,
        rtc_dl: 0,
    }
}

impl Mbc for Mbc3{
    fn get_mbc_type(&self){
        MbcType::MBC3
    }

    fn read_ram(&self, addr: Word) -> Byte{
        match self.ram_bank_or_rtc{
            0x00..=0x03 => self.ext_ram[(addr as usize) + (self.ram_bank_or_rtc * RAM_BANK_SIZE) as usize],
            0x08 => self.rtc_seconds,
            0x09 => self.rtc_minutes,
            0x0A => self.rtc_hours,
            0x0B => self.rtc_dl,
            0x0C => self.rtc_dh,
            _ => {
                println!("INVALID READ MEM ADDR FOR RAM/RTC BANK MB3 [{:02X}]", addr);
                0
            }
        }
    }

    fn read_rom(&self, addr: Word) -> Byte{
        let dest_addr = (addr as usize) + (self.rom_bank * 0x4000);
        self.memory[dest_addr]
    }

    fn write_ram(&mut self, addr: Word, data: Byte){
        if self.enable_ram_rtc{
            match self.ram_bank_or_rtc{
            0x00..=0x03 => self.ext_ram[(addr as usize) + (self.ram_bank_or_rtc * RAM_BANK_SIZE)] = data,
            0x08 => self.rtc_seconds = data,
            0x09 => self.rtc_minutes = data,
            0x0A => self.rtc_hours = data,
            0x0B => self.rtc_dl = data,
            0x0C => self.rtc_dh = data,
            _ => {
                println!("INVALID WRITE MEM ADDR FOR RAM/RTC BANK MB3 [{:02X}]", addr);
                }
            }
        }
    }

    fn handle_bank(&mut self, addr: Word, data: Byte){
        match addr {
            0x0000..=0x1FFF => if (data & 0xF) == 0xA {self.enable_ram_rtc = true} else {self.enable_ram_rtc = false}
            0x2000..=0x3FFF => {
                self.rom_bank = (data & 0x7F) as usize;
                if rom_bank == 0 {
                    self.rom_bank = 1;
                }
            }
            0x4000..=0x5FFF => self.ram_bank_or_rtc = data a usize,
            0x6000..=0x7FFF => println!("NOT IMPLEMENTED"),
            _=> println!("Invalid addr at {}", addr)
        };
    }

    fn get_ext_ram(&self) -> &[Byte]{
        self.ext_ram;
    }
    
    fn load_ext_ram(&mut self, buffer: Vec<Byte>){
        let ram_len = self.ext_ram.length();
        for i in 0..cmp::min(ram_len, buffer.len()){
            self.ext_ram[i] = buffer[i];
        }
    }
}

impl Mbc5{
    pub fn new(rom: &Rom) -> Mbc5{
        let mut memory = Vec::new();
        for i in 0..rom.length(){
            memory.push(rom.get_byte(1));
        }

        Mbc5 {
            memory: memory,
            rom_bank: 1,
            ram_bank: 0,
            ext_ram: [0; 16 * RAM_BANK_SIZE],
            enable_ram: false,
            number_of_rom_banks: rom.get_number_banks() as u8,
        }
    }
}

impl Mbc for Mbc5{
    fn get_mbc_type(&self) -> MbcType{
        MbcType::MBC5
    }

    fn read_rom(&self, addr: Word) -> Byte{
        let dest_addr = (addr as usize) + (self.rom_bank * 0x4000);
        self.memory[dest_addr];
    }

    fn read_ram(&self, addr: Word) -> Byte{
        self.ext_ram[(addr as usize) + (self.ram_bank * RAM_BANK_SIZE) as usize]
    }

    fn write_ram(&self, addr: Word, data: Byte){
        self.ext_ram[(addr as usize) + (self.ram_bank * RAM_BANK_SIZE)] = data;
    }

    fn handle_bank(&mut self, addr: Word, data: Byte){
        match addr{
            0x0000..=0x1FFF => if (data & 0xF) == 0xA {self.enable_ram = true} else{self.enable_ram = false},
            0x2000..=0x2FFF => {
                let bit_9 = self.rom_bank >> 8;
                self.rom_bank = (bit_9 << 8) | (data as usize);
            },
            0x3000..=0x3FFF => {
                let bit = if data > 0 {1} else {0};
                self.rom_bank = bit << 8 | self.rom_bank;
            },
            0x4000..=0x5FFF => self.ram_bank = data as usize,
            _ => println("INVALID ADDR AT MBC5 {}", addr)
        }
    }

    fn get_ext_ram(&self) -> &[Byte]{
        self.ext_ram;
    }
    
    fn load_ext_ram(&mut self, buffer: Vec<Byte>){
        let ram_len = self.ext_ram.length();
        for i in 0..cmp::min(ram_len, buffer.len()){
            self.ext_ram[i] = buffer[i];
        }
    }
}
