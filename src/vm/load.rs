use super::{Result, Vm, VmError};

impl Vm {
    pub fn _load(&mut self) -> Result<()> {
        let dest = self.next_8()?;
        let value = self.next_16()?;

        if dest > 31 {
            return Err(VmError::InvalidRegister(dest));
        }

        self.registers[dest as usize] = value as i64;

        Ok(())
    }
}
