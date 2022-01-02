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
        }
    }
}