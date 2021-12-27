/*

The gameboy CPU has 8 registers, namely A through L. Since the Sharp LR35902 is a 8 bit CPU based off Z80 and Intel 8080, each register operates 8bit (1 byte)

Each register receives an unsigned 8 bit integer to operate. Data types are described here in Rust:
https://doc.rust-lang.org/book/ch03-02-data-types.html

Why are they unsigned? Because unsigned values can't be negative.

*/

// a,b,c,d,e,h,l are general registers
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8, // Flag Register
    h: u8,
    l: u8,
    pc: u16, //Program Counter, 16bit
    sp: u16, //Stack Pointer, 16bit
}

/*

Although there are 8 registers, the Sharp LR35902 allowed for RW operations at the same time, which means that there is a implementation on 16bit registers denoted as AF, for instance.

This is a implementation of RW 16bits operations. Here's how methods are handled in Rust:
https://doc.rust-lang.org/book/ch05-03-method-syntax.html

Also a little bit about bitwise operations:
https://doc.rust-lang.org/book/appendix-02-operators.html

A little bit more about 16bit registers:
https://gbdev.io/pandocs/CPU_Registers_and_Flags.html

*/
impl Registers {
        
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b =  ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16{
        (self.d as u16) << 8 | self.e as u16
    }
    fn set_de(&mut self, value: u16){
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16{
        (self.h as u16) << 8 | self.l as u16
    }
    fn set_hl(&mut self, value: u16){
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

}

struct FlagRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagRegister> for u8 {
    fn from(flag: FlagRegister) -> u8 {
        (if flag.zero           {1} else {0}) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract       {1} else {0}) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry     {1} else {0}) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry          {1} else {0}) << CARRY_FLAG_BYTE_POSITION 
    }
}

impl std::convert::From<u8> for FlagRegister{
    fn from(byte: u8) -> Self{
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) !=0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION & 0b1)) !=0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION & 0b1)) !=0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION & 0b1)) !=0;
    }

    FlagRegister{
        zero,
        subtract,
        half_carry,
        carry
    }
}

enum Instruction{
    ADD(pointerTarget),
    SUBC(pointerTarget),
    AND(pointerTarget),
    XOR(pointerTarget),
    INC(pointerTarget),
    DEC(pointerTarget),
    OR(pointerTarget),
    CP(pointerTarget),
}

enum pointerTarget{
    A,B,C,D,E,H,L,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction){
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    pointerTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        //TO DO:Implement instruction
                    }
                }
            }
        }
    }

    fn add(&mut self, value:u8) -> u8{
        let (new_value, did_overflow) = self.registers.a.did_overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.subtract= false;
        new_value;
    }
}

