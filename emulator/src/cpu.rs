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

    fn fetchbyte(&mut self) -> u8{
        let b = self.mmu.b(self.rg.pc);
        self.rg.pc +=1
    }

    fn add(&mut self, b:u8, usec:bool){
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

    fn call(&mut self) -> u32 {
        let opcode = self.fetchbyte();
        match opcode{
            0x00 => {
                1
            }
            0x01 =>{
                let v = self.fetchword();
                self.reg.setbc(v);
                3   
            }
            0x02 =>{

            }
            0x03 =>{
                
            }
            0x04 =>{
                
            }
            0x05 =>{
                
            }
            0x06 =>{
                
            }
            0x07 =>{
                
            }
            0x08 =>{
                
            }
            0x09 =>{
                
            }
            0x0A =>{
                
            }
            0x0B =>{
                
            }
            0x0C =>{
                
            }
            0x0D =>{
                
            }
            0x0E =>{
                
            }
            0x0F =>{
                
            }
            0x76 =>{
                self.halted = true; 1   
            }
        }
    }
}