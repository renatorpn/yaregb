// this is the implementation of the memory management unit
// mmu is the interface between cpu, ppu, spu and the memory. this is the control bus


//wram is the working ram
const wram_size: usize = 0x8000;
const zram_size: usize = 0x7f;

pub struct mmu<'a> {
    wram: [u8; wram_size],
    zram: [u8; zram_size],
}