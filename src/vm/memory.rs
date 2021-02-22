use super::{Result, Vm, VmError};

impl Vm {
    pub fn _rqm(&mut self) -> Result<()> {
        let register = self.next_8()?;

        if register > 31 {
            return Err(VmError::InvalidRegister(register));
        }

        let bytes = self.next_16()?;

        self.registers[register as usize] = self.heap.len() as i64;

        self.heap.resize(self.heap.len() + bytes as usize, 0);
        Ok(())
    }

    pub fn _ascii(&mut self) -> Result<()> {
        let register = self.next_8()?;

        println!("Register: {:#02x}", register);

        if register > 31 {
            return Err(VmError::InvalidRegister(register));
        }

        let mut amount = 1;
    
        while self.program[self.ip + amount] != 0x00 && self.ip + amount < self.program.len() {
            amount += 1;
        }

        let start = self.heap.len();

        // allocate memory (do not forget 0x00 at the end)
        self.heap.resize(self.heap.len() + amount + 1, 0);

        for i in 0..amount {
            self.heap[start + i] = self.next_8()?;
        }
        self.heap[start + amount] = 0x00;

        self.registers[register as usize] = start as i64;

        Ok(())
    }
}
