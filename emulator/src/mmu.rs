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

pub fn reset(&mut self){

}