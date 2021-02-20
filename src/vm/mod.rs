use std::fmt::{self, Debug, Formatter};

pub mod load;
pub mod moove;

pub struct Vm {
    registers: [i64;32],
    ip: usize,
    program: Vec<u8>,
    state: bool,
}

// Constants
pub const LOAD: u8 = 0x00;
pub const MOVE: u8 = 0x01;

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