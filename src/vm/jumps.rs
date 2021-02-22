use super::{Result, Vm, VmError};

impl Vm {
    pub fn _jmp(&mut self) -> Result<()> {
        let value = self.next_16()?;

        if value as usize >= self.program.len() {
            return Err(VmError::OutOfBounds);
        }

        self.ip = value as usize;

        Ok(())
    }
    pub fn _jmpeq(&mut self) -> Result<()> {
        let value = self.next_16()?;

        if value as usize >= self.program.len() {
            return Err(VmError::OutOfBounds);
        }

        if self.registers[31] == 1 {
            self.ip = value as usize;
        }

        Ok(())
    }

    pub fn _rjmp(&mut self) -> Result<()> {
        let value = self.next_16()?;

        if self.ip + value as usize >= self.program.len() {
            return Err(VmError::OutOfBounds);
        }
        self.ip += value as usize;

        Ok(())
    }
}
