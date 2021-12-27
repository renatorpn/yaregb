
// a,b,c,d,e,h,l are general registers
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    f: u8, // Flag Register
    pub h: u8,
    pub l: u8,
    pub pc: u16, //Program Counter, 16bit
    pub sp: u16, //Stack Pointer, 16bit
}


// see https://dev.to/abhinavmir/gameboy-everything-under-the-hood-1p0b for why this values in binary
pub enum CpuFlags{
    C = 0b00010000, //Carry flag
    H = 0b00100000, //Half Carry flag
    N = 0b01000000, //Subtract flag
    Z = 0b10000000, //Zero flag
}

//why these values, dawg?
impl Registers {
    pub fn new() -> Registers {
        Registers {
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

    
    pub fn af(&self) -> Registers {
        ((self.a as u16) << 8) | ((self.f & 0xF0) as u16)
    }

    pub fn bc() -> Registers {

    }

    pub fn de() -> Registers {
        
    }

    pub fn hl() -> Registers {
        
    }

    pub fn sp() -> Registers {
        
    }

    pub fn pc() -> Registers {
        
    }
}

