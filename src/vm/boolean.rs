use super::{Result, Vm, VmError};

impl Vm {
    pub fn _or(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        let lh = self.registers[lhs as usize] != 0;
        let rh = self.registers[rhs as usize] != 0;

        self.registers[31] = (lh | rh) as i64;

        Ok(())
    }

    pub fn _and(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        let lh = self.registers[lhs as usize] != 0;
        let rh = self.registers[rhs as usize] != 0;

        self.registers[31] = (lh & rh) as i64;

        Ok(())
    }

    pub fn _xor(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        let lh = self.registers[lhs as usize] != 0;
        let rh = self.registers[rhs as usize] != 0;

        self.registers[31] = (lh ^ rh) as i64;

        Ok(())
    }

    pub fn _not(&mut self) -> Result<()> {
        let reg = self.next_8()?;

        if reg > 31 {
            return Err(VmError::InvalidRegister(reg));
        }

        self.registers[31] = (self.registers[reg as usize] == 0) as i64;

        Ok(())
    }

    pub fn _eq(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        self.registers[31] = (self.registers[lhs as usize] == self.registers[rhs as usize]) as i64;

        Ok(())
    }

    pub fn _neq(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        self.registers[31] = (self.registers[lhs as usize] != self.registers[rhs as usize]) as i64;

        Ok(())
    }

    pub fn _leq(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        self.registers[31] = (self.registers[lhs as usize] <= self.registers[rhs as usize]) as i64;

        Ok(())
    }

    pub fn _le(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        self.registers[31] = (self.registers[lhs as usize] < self.registers[rhs as usize]) as i64;

        Ok(())
    }

    pub fn _geq(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        self.registers[31] = (self.registers[lhs as usize] >= self.registers[rhs as usize]) as i64;

        Ok(())
    }

    pub fn _ge(&mut self) -> Result<()> {
        let lhs = self.next_8()?;
        let rhs = self.next_8()?;

        if lhs > 31 {
            return Err(VmError::InvalidRegister(lhs));
        } else if rhs > 31 {
            return Err(VmError::InvalidRegister(rhs));
        }

        self.registers[31] = (self.registers[lhs as usize] > self.registers[rhs as usize]) as i64;

        Ok(())
    }
}
