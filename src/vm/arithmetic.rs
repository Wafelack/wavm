use super::{Result, Vm, VmError};

impl Vm {
    pub fn _add(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        self.registers[lhs as usize] += self.registers[rhs as usize];

        Ok(())
    }
    pub fn _sub(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        self.registers[lhs as usize] -= self.registers[rhs as usize];

        Ok(())
    }
    pub fn _mul(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        self.registers[lhs as usize] *= self.registers[rhs as usize];

        Ok(())
    }
    pub fn _div(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        self.registers[lhs as usize] /= self.registers[rhs as usize];

        Ok(())
    }
    pub fn _mod(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        self.registers[lhs as usize] %= self.registers[rhs as usize];

        Ok(())
    }
}
