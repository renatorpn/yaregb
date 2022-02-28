use std::cmp;

use crate::joypad::*;
use crate::mbc::*;
use crate::rom::*;
use crate::utils::*;

// this is the implementation of the memory management unit
// mmu is the interface between cpu, ppu, spu and the memory. this is the control bus


/**
    * Memory Management Unit for the Gameboy. Memory has a 16 bit address bus and is broken down as follows:
    *    0000 - 3FFF	    16 KiB ROM bank 00	            From cartridge, usually a fixed bank
    *    4000 - 7FFF	    16 KiB ROM Bank 01~NN	        From cartridge, switchable bank via mapper (if any)
    *    8000 - 9FFF	    8 KiB Video RAM (VRAM)	        In CGB mode, switchable bank 0/1
    *    A000 - BFFF	    8 KiB External RAM	            From cartridge, switchable bank if any
    *    C000 - CFFF	    4 KiB Work RAM (WRAM)
    *    D000 - DFFF	    4 KiB Work RAM (WRAM)	        In CGB mode, switchable bank 1~7
    *    E000 - FDFF	    Mirror of C000~DDFF (ECHO RAM)	Nintendo says use of this area is prohibited.
    *    FE00 - FE9F	    Sprite attribute table (OAM)
    *    FEA0 - FEFF	    Not Usable	                    Nintendo says use of this area is prohibited
    *    FF00 - FF7F	    I/O Registers
    *    FF80 - FFFE	    High RAM (HRAM)
    *    FFFF - FFFF	    Interrupt Enable register (IE)
    **/



//wram is the working ram
const wram_size: usize = 0x8000;
const zram_size: usize = 0x7f;

pub struct mmu {
    memory: [Byte; memory_size],
    oam: bool,
    vram: bool,
    color_pallette: bool,
    rom: Rom,
    joypad: Joypad,
    mbc: Option<Box<dyn Mbc>>
}

impl Mmu{

    pub fn init(rom: Rom, joypad: Joypad) -> Mmu
    {
        Mmu{
            memory: [0; memory_size],
            oam: true,
            vram: true,
            color_pallette: true,
            rom: rom,
            joypad: joypad,
            mbc: None     
        }   
    }

    pub fn get_ext_ram(&self) -> &[Byte]{
        match &self.mbc{
            Some(mbc) => mbc.get_ext_ram();
            None => format!("MBC Type: None"),
        }
    }

    pub fn load_ext_ram(&self) -> &[Byte]{
        match &mut self.mbc{
            Some(mbc) => mbc.load_ext_ram(buffer),
            None => {
                let ram_length = 0xC000 - 0xA000;
                for i in 0..cmp:min(ram_length, buffer.len()){
                    self.memory[0xA000 + i] + buffer[i];
                }
            }
        }

    }

    pub fn bus_read(&self, address:u16) -> u8{
        let address;
        if address < 0x8000 {
            self.read_rom(address)
        }else if address >= 0x8000 && address < 0xC000  {
            self.read_ram(address)
        }else {
            self.memory[address as usize]
        }
    }
    pub fn reset(&mut self){
        self.memory[0xFF05] = 0x00;
        self.memory[0xFF06] = 0x00;
        self.memory[0xFF07] = 0x00;
        self.memory[0xFF10] = 0x00;
        self.memory[0xFF11] = 0x00;
        self.memory[0xFF12] = 0x00;
        self.memory[0xFF14] = 0x00;
        self.memory[0xFF16] = 0x00;
        self.memory[0xFF17] = 0x00;
        self.memory[0xFF19] = 0x00;
        self.memory[0xFF1A] = 0x00;
        self.memory[0xFF1B] = 0x00;
        self.memory[0xFF1E] = 0x00;
        self.memory[0xFF20] = 0x00;
        self.memory[0xFF21] = 0x00;
        self.memory[0xFF22] = 0x00;
        self.memory[0xFF23] = 0x00;
        self.memory[0xFF24] = 0x00;
        self.memory[0xFF25] = 0x00;
        self.memory[0xFF26] = 0x00;
        self.memory[0xFF40] = 0x00;
        self.memory[0xFF42] = 0x00;
        self.memory[0xFF43] = 0x00;
        self.memory[0xFF45] = 0x00;
        self.memory[0xFF47] = 0x00;
        self.memory[0xFF48] = 0x00;
        self.memory[0xFF49] = 0x00;
        self.memory[0xFF4A] = 0x00;
        self.memory[0xFF4B] = 0x00;
        self.memory[0xFFFF] = 0x00;
    
        self.memory[joypad_register_address as usize] = 0xFF;
        self.load_rom();
    
    }
    
    // may seem strange, but some carts do have access to write to ROM (like lithium battery)
    pub fn bus_write(&self, address:u16 ) -> u8{
        let address;
        if address < 0x8000 {
            self.write_rom(address)
        }else if address >= 0x8000 && address < 0xC000  {
            self.read_ram(address)
        }else {
            self.memory[address as usize]
        }
    }

    fn load_rom(&mut self){
        let end_address = 0x8000;
        for i in 0..cmp::min(end_address, self.rom.lenght()){
            self.memory[i] = self.rom.get_byte(i);
        }
        self.mbc = get_mbc(&self.rom)
    }

    fn read_rom(&self, addr: Word) -> Byte{
        match &self.mbc{
            Some(mbc) => mbc.read_rom(addr - 0xA000);
            None => self.rom.get_byte(addr as usize),
        }
    }

    fn read_ram(&self, addr: Word) -> Byte{
        match &self.mbc{
            Some(mbc) => mbc.read_ram(addr - AxA000, data),
            None(mbc) => self.memory[addr as usize],
        }
    }

    fn write_ram(&mut self, addr: Word, data: Byte){
        match &mut self.mbc{
            Some(mbc) => mbc.write_ram(addr - AxA000, data),
            None(mbc) => self.memory[addr as usize] = data,
        }
    } 

    fn handle_bank(&mut self, addr: Word, data: Byte){
        match &mut self.mbc{
            Some(mbc) => mbc.handle_bank(addr, data);
            None => {},
        }
    }

    //I don't understand this, why 0x3?
    fn handle_joypad(&mut self, addr: Word, data: Byte){
        let mode_bits = (data >> 4) & 0x3;
        let mode = match mode_bits {
            1 => Some(Joypad::ACTION), 
            2 => Some(Joypad::DIRECTION),
            _ => None
        };

        if let Some(joypad_mode) = mode{
            let low_4bits = self.joypad.get_buttons_for_mode(joypad_mode);
            self.memory[addr as usize] = data (data & 0xF0) | low_4bits;
        }else{
            self.memory[addr as usize] = data (data & 0xF0) | 0xF0;
        }
    }

    pub fn update_timer_freq(&mut self, val:bool){
        self.timer_freq_changed = val;
    }

    pub fn timer_freq_is_changed(&mut self) -> bool{
        self.timer_freq_changed
    }

    pub fn update_scanline(&mut self){
        self.memory[CURRENT_SCANLINE_ADDR as usize] = self.memory[CURRENT_SCANLINE_ADDR as usize].wrapping_add(1);
    }

    pub fn reset_scanline(&mut self){
        self.memory[CURRENT_SCANLINE_ADDR as usize] = 0;
    }

    pub fn increment_timer_register(&mut self){
        self.memory[TIMER_ADDR as usize] = self.memory[TIMER_ADDR as usize].wrapping_add(1);
    }

    pub fn increment_division_register(&mut self){
        self.memory[DIVIDER_REGISTER_ADDR as usize] = self.memory[DIVIDER_REGISTER_ADDR as usize].wrapping_add(1);
    }

    pub fn set_button_state(&mut self, button:usize){
        self.joypad.set_button_state(button);
    }

    pub fn reset_button_state(&mut self, button:usize){
        self.joypad.reset_button_state(button);
    }

    fn timer_control(&mut self, data:Byte){
        self.update_timer_freq(true);
        self.memory[TIMER_CONTROL_ADDR as usize] = data;
    }

}


