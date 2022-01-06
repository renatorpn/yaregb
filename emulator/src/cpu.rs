use crate::register::cpu_flags::{C, N, H, Z};
use crate::register::registers;
use crate::mmu::mmu;




pub struct cpu<'a>{
    reg: registers,
    pub mmu: mmu<'a>,
    halted: bool,
    ime: bool,
    setdi: u32,
    setei: u32,
    mmu: cpu_mmu,
}

impl<'a> cpu<'a>{
    pub fn new(romname: &str, serial_callback: Option<SerialCallback<'a>>, skip_checksum: bool) -> StrResult<cpu<'a>>{
        let cpu_mmu = mmu::new(romname, serial_callback, skip_checksum)?;
        Ok(cpu {
            reg: registers::new(),
            halted: false,
            ime: true,
            setdi: 0,
            setei: 0,
            mmu: cpu_mmu,
        })
    }

    pub fn do_cycle(&mut self) -> u32 {
        let ticks = self.cycle() * 4;
        return self.mmu.do_cycle(ticks)
    }

    fn cycle(&mut self) -> u32{
        self.updatetime();
        match self.handleinterrupt(){
            0 => {},
            n => return n,
        };

        if self.halted {
            1
        }else{
            self.call()
        }
    }

    fn fetch_next(&mut self) -> u8{
        let b = self.mmu.b(self.rg.pc);
        self.rg.pc +=1
    }

    // read about ref and & (https://doc.rust-lang.org/std/keyword.ref.html)
    fn load_cmd(&mut self, opcode: &OpCode) -> u8{
        match opcode{
            //these implementations are probably quite wrong... I have to check them later
            
            
            //8 bit
            //LD nn,n
            0x06 => reg.b = self.fetch_byte(),
            0x0E => reg.c = self.fetch_byte(),
            0x16 => reg.d = self.fetch_byte(),
            0x1E => reg.e = self.fetch_byte(),
            0x26 => reg.h = self.fetch_byte(),
            0x2E => reg.l = self.fetch_byte(),
            
            //LD r1,r2
            0x7F => reg.a = reg.a,
            0x78 => reg.a = reg.b,
            0x79 => reg.a = reg.c,
            0x7A => reg.a = reg.d,
            0x7B => reg.a = reg.e,
            0x7C => reg.a = reg.h,
            0x7D => reg.a = reg.l,
            0x7E => reg.a = reg.hl,
            0x40 => reg.b = reg.b,
            0x41 => reg.b = reg.c,
            0x42 => reg.b = reg.d,
            0x43 => reg.b = reg.e,
            0x44 => reg.b = reg.h,
            0x45 => reg.b = reg.l,
            0x46 => reg.b = reg.hl,
            0x48 => reg.c = reg.b,
            0x49 => reg.c = reg.c,
            0x4A => reg.c = reg.d,
            0x4B => reg.c = reg.e,
            0x4C => reg.c = reg.h,
            0x4D => reg.c = reg.l,
            0x4E => reg.c = reg.hl,
            0x50 => reg.d = reg.b,
            0x51 => reg.d = reg.c,
            0x52 => reg.d = reg.d,
            0x53 => reg.d = reg.e,
            0x54 => reg.d = reg.h,
            0x55 => reg.d = reg.l,
            0x56 => reg.d = reg.hl,
            0x58 => reg.e = reg.b,
            0x59 => reg.e = reg.c,
            0x5A => reg.e = reg.d,
            0x5B => reg.e = reg.e,
            0x5C => reg.e = reg.h,
            0x5D => reg.e = reg.l,
            0x5E => reg.e = reg.hl,
            0x60 => reg.h = reg.b,
            0x61 => reg.h = reg.c,
            0x62 => reg.h = reg.d,
            0x63 => reg.h = reg.e,
            0x64 => reg.h = reg.h,
            0x65 => reg.h = reg.l,
            0x66 => reg.h = reg.hl,
            0x68 => reg.l = reg.b,
            0x69 => reg.l = reg.c,
            0x6A => reg.l = reg.d,
            0x6B => reg.l = reg.e,
            0x6C => reg.l = reg.h,
            0x6D => reg.l = reg.l,
            0x6E => reg.l = reg.hl,
            0x70 => reg.hl = reg.b,
            0x71 => reg.hl = reg.c,
            0x72 => reg.hl = reg.d,
            0x73 => reg.hl = reg.e,
            0x74 => reg.hl = reg.h,
            0x75 => reg.hl = reg.l,
            0x36 => reg.hl = self.fetch_byte(),

            //LD A,n
            0x0A => reg.a = reg.bc,
            0x1A => reg.a = reg.de,
            0x7E => reg.a = reg.hl,
            0xFA => reg.a = self.fetch_word(),
            0x3E => reg.a = self.fetch_byte(),

            //LD n,A
            0x4F => reg.a = reg.c,
            0x5F => reg.a = reg.e,
            0x6F => reg.a = reg.l,
            0x02 => reg.a = reg.bc,
            0x12 => reg.a = reg.de,
            0x77 => reg.a = reg.hl,
            0xEA => reg.a = self.fetch_word(),



            0x01 => reg.setbc = self.fetch_next();
            0x11 => reg.setde = self.fetch_next();
            0x21 => reg.sethl = self.fetch_next();
            0x31 => reg.sp = self.fetch_next();

            

        }

    }

    fn pop_cmd(){

    }

    fn add_cmd(&mut self, b:u8, usec:bool){
        let c;
        if usec && self.reg.getflag(C){
            let c = 1;
        }else{
            let c = 0
        }
        let a = self.reg.a;
        let r = a.wrapping_add(b).wrapping_add(c);
        self.reg.flag(Z, r == 0)
        self.reg.flag(H, (a & 0xF) + (b & 0xF) + c > 0xF)
        self.reg.flag(N, false)
        self.reg.flag(C, (a as u16) + (b as u16) + (c as u16) > 0xFF)
        self.reg.a = r;
    }

    fn adc_cmd(){

    }

    fn sub_cmd(){

    }

    fn sbc_cmd(){

    }

    fn and_cmd(){

    }

    fn or_cmd(){

    }

    fn xor_cmd(){

    }

    fn cp_cmd(){

    }

    fn inc_cmd(){

    }

    fn dec_cmd(){

    }

    fn swap_cmd(){

    }

    fn jump_cmd(){

    }

    fn call(&mut self) -> u32 {
        let opcode = self.fetchbyte();
        match opcode{
            0x76 =>{
                self.halted = true; 1   
            }
        }
    }
}