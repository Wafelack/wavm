use std::fmt::{self, Debug, Formatter};

pub mod load;
pub mod moove;
pub mod arithmetic;
pub mod boolean;
pub mod jumps;

pub struct Vm {
    registers: [i64;32],
        // 32th: Eq register
    ip: usize,
    program: Vec<u8>,
    state: bool,
}

// Constants
pub const LOAD: u8 = 0x00;
pub const MOVE: u8 = 0x01;

pub const ADD: u8 = 0x02;
pub const SUB: u8 = 0x03;
pub const MUL: u8 = 0x04;
pub const DIV: u8 = 0x05;
pub const MOD: u8 = 0x06;

pub const EQ: u8 = 0x07;
pub const GEQ: u8 = 0x08;
pub const GE: u8 = 0x09;
pub const LEQ: u8 = 0x0A;
pub const LE: u8 = 0x0B;
pub const NEQ: u8 = 0x0C;
pub const NOT: u8 = 0x0D;
pub const OR: u8 = 0x0E;
pub const AND: u8 = 0x0F;
pub const XOR: u8 = 0x10;

pub const JMP: u8 = 0x11;
pub const JMPEQ: u8 = 0x12;
pub const RJMP: u8 = 0x13;

pub const HLT: u8 = 0xCC;

impl Vm {
    pub fn new(program: Vec<u8>) -> Self {
        Self {
            program,
            registers: [0;32],
            ip: 0,
            state: true, // running
        }
    }
    
    pub fn get_registers(&self) -> [i64;32] {
        self.registers
    }

    pub fn run(&mut self) -> Result<()> {

        while self.ip < self.program.len() && self.state {
            self._run_cycle()?;
        }
        self.state = false;
        Ok(())
    }
    fn _run_cycle(&mut self) -> Result<()> {
        let opcode = self.program[self.ip];
        match opcode {
            LOAD => self._load()?,
            MOVE => self._move()?,

            ADD => self._add()?,
            SUB => self._sub()?,
            MUL => self._mul()?,
            DIV => self._div()?,
            MOD => self._mod()?,

            EQ => self._eq()?,
            NEQ => self._neq()?,
            GEQ => self._geq()?,
            GE => self._ge()?,
            LEQ => self._leq()?,
            LE => self._le()?,
            NOT => self._not()?,
            AND => self._and()?,
            OR => self._or()?,
            XOR => self._xor()?,

            JMP => self._jmp()?,
            JMPEQ => self._jmpeq()?,
            RJMP => self._rjmp()?,

            HLT => self.state = false,
            _ => return Err(VmError::InvalidOpcode(opcode))
        }

        self.ip += 1;
        Ok(())
    }

    // utils
    fn next_8(&mut self) -> Result<u8> {
        if self.ip >= self.program.len() {
            Err(
                VmError::OutOfBounds
            )
        } else {
            self.ip += 1;
            Ok(
                
                self.program[self.ip]
            )
        }
    }
    fn next_16(&mut self) -> Result<u16> {
        if self.ip + 1 >= self.program.len() {
            Err(
                VmError::OutOfBounds
            )
        } else {
            self.ip += 1;
            self.ip += 1;
            Ok(
                
                (self.program[self.ip - 1] as u16) << 8 | self.program[self.ip] as u16
            )
        }
    }
}

pub enum VmError {
    InvalidOpcode(u8),
    OutOfBounds,
    InvalidRegister(u8),
}

impl Debug for VmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        
        match self {
            Self::InvalidOpcode(opc) => write!(f, "Invalid opcode: {}.", opc)?,
            Self::InvalidRegister(reg) => write!(f, "Invalid register: {:#02x}", reg)?,
            Self::OutOfBounds => write!(f, "Out of bounds indexing.")?,
        }

        Ok(())
    }
}

pub type Result<T> = std::result::Result<T, VmError>;

mod tests;