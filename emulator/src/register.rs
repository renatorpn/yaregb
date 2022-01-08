/*

The gameboy CPU has 8 registers, namely A through L. Since the Sharp LR35902 is a 8 bit CPU based off Z80 and Intel 8080, each register operates 8bit (1 byte)

Each register receives an unsigned 8 bit integer to operate. Data types are described here in Rust:
https://doc.rust-lang.org/book/ch03-02-data-types.html

Why are they unsigned? Because unsigned values can't be negative.

*/


/*

Although there are 8 registers, the Sharp LR35902 allowed for RW operations at the same time, which means that there is a implementation on 16bit registers denoted as AF, for instance.

This is a implementation of RW 16bits operations. Here's how methods are handled in Rust:
https://doc.rust-lang.org/book/ch05-03-method-syntax.html

Also a little bit about bitwise operations:
https://doc.rust-lang.org/book/appendix-02-operators.html

A little bit more about 16bit registers:
https://gbdev.io/pandocs/CPU_Registers_and_Flags.html

*/

// https://gbdev.io/pandocs/CPU_Registers_and_Flags.html


// for more info, check the Z80 manual https://www.zilog.com/docs/z80/um0080.pdf
pub struct registers {
    pub a: u8, //Accumulator
    pub b: u8, //Accumulator
    pub c: u8, //Accumulator
    pub d: u8, //Accumulator
    pub e: u8, //Accumulator
    f: u8, // Flag Register
    pub h: u8, //Accumulator
    pub l: u8, //Accumulator
    pub pc: u16, //Special Purpose Register - Program Counter, 16bit
    pub sp: u16, //Special Purpose Register - Stack Pointer, 16bit
}


// see https://dev.to/abhinavmir/gameboy-everything-under-the-hood-1p0b for why this values in binary
// this is a 8bit representation of the register, so C->4, H->5, N->6, Z->7
//
// |  7  |  6  |  5  |  4  |  3  |  2  |  1  |  0  |
// |  Z  |  N  |  H  |  C  |  0  |  0  |  0  |  0  |
//
pub enum cpu_flags{
    C = 0b00010000, //Carry flag 
    H = 0b00100000, //Half Carry flag
    N = 0b01000000, //Subtract flag
    Z = 0b10000000, //Zero flag
}

//why these values, dawg?
impl registers {
    pub fn new() -> registers {
        registers {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0xB0,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    // this code is based on mvdnes rboy implementation, see: https://github.com/mvdnes/rboy/blob/master/src/register.rs
    
    pub fn af(&self) -> u16 {
        // this is a function and closure return type, see more:
        // https://doc.rust-lang.org/book/appendix-02-operators.html
        // "&" is a bitwise AND operator
        // "<<" is a left shift
        // https://www.tutorialspoint.com/rust/rust_bitwise_operators.htm
        ((self.a as u16) << 8) | ((self.f & 0xF0) as u16)
    }

    // General purpose register
    pub fn bc() -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    // General purpose register
    pub fn de() -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    // General purpose register
    pub fn hl() -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    // not exactly sure this is the right implementation for the stack_pointer register
    pub fn sp() -> u16 {
        self.sp as u16
    }

    // not exactly sure this is the right implementation for the program_counter register
    pub fn pc() -> u16 {
        self.pc as u16
    }

    // hl register Decrement
    pub fn hld() -> u16 {
        let res = self.hl();
        self.sethl(res - 1);
        res
    }

    // hl register Increment
    pub fn hli() -> u16 {
        let res = self.hl();
        self.sethl(res + 1);
        res
    }

    pub fn setaf(&mut self, value:u16){
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00F0) as u8;
    }

    pub fn setbc(&mut self, value:u16){
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn setde(&mut self, value:u16){
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn sethl(&mut self, value:u16){
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    pub fn flag(&mut self, flags: cpu_flags, set: bool){
        let mask = flags as u8;
        match set {
            true = self.f |= mask,
            false = self.f &= !mask,
        }
        self.f &= 0xF0
    }

    pub fn getflag(&self, flags: cpu_flags) -> bool{
        let mask = flags as u8;
        self.f & mask > 0
    }

    /* 
    // Stack Pointer register (stack structure)
    // the stack in GB keeps the variables and return addresses in the memory in a FIFO fashion. Passes arguments to subroutines.
    // to push we use PUSH, CALL and RST instructions. to remove we use POP, RET and RETI.
    // SP keeps track of the top of the stack.
    // see: https://dev.to/abhinavmir/gameboy-everything-under-the-hood-1p0b 
    pub fn sp() -> registers {
        
    }

    pub fn pc() -> registers {
        
    }*/
}

