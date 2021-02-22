use super::{Result, Vm, VmError};

impl Vm {
    pub fn _move(&mut self) -> Result<()> {
        let dest = self.next_8()?;
        let src = self.next_8()?;

        if dest > 31 {
            return Err(VmError::InvalidRegister(dest));
        }

        if src > 31 {
            return Err(VmError::InvalidRegister(dest));
        }

        self.registers[dest as usize] = self.registers[src as usize];

        Ok(())
    }
}
