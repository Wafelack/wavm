use super::{Result, Vm, VmError};

impl Vm {
    pub fn _get_byte(&mut self) -> Result<()> {

        let dest_reg = self.next_8()?;

        if dest_reg > 31 {
            return Err(VmError::InvalidRegister(dest_reg));
        }

        let ptr_reg = self.next_8()?;

        if ptr_reg > 31 {
            return Err(VmError::InvalidRegister(ptr_reg));
        }

        let ptr = self.registers[ptr_reg as usize] as usize;

        if ptr >= self.heap.len() {
            Err(
                VmError::SegmentationFault
            )
        } else {

            self.registers[dest_reg as usize] = self.heap[ptr_reg as usize] as i64;

            Ok(())
        }

        
    }
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

    pub fn _setbyte(&mut self) -> Result<()> {
        let ptr = self.next_8()?;

        if ptr > 31 {
            return Err(VmError::InvalidRegister(ptr))
        }

        let value = self.next_8()?;

        if value > 31 {
            return Err(VmError::InvalidRegister(value))
        }

        let ptr_val = self.registers[ptr as usize] as usize;

        if ptr_val >= self.heap.len() {
            return Err(
                VmError::SegmentationFault
            )
        }

        let value_val = self.registers[value as usize];

        self.heap[ptr_val] = if value_val > 255 {
            255
        } else if value_val < 0 {
            0
        } else {
            value_val as u8
        };

        Ok(())
    }

    pub fn _free(&mut self) -> Result<()> {
        let ptr = self.next_8()?;

        if ptr > 31 {
            return Err(VmError::InvalidRegister(ptr))
        }

        let value = self.next_8()?;

        if value > 31 {
            return Err(VmError::InvalidRegister(value))
        }

        let ptr_val = self.registers[ptr as usize] as usize;

        let value_val = self.registers[value as usize] as usize;

        if ptr_val  + value_val >= self.heap.len() {
            return Err(
                VmError::OutOfBounds
            )
        }

        for i in 0..value_val {
            self.heap[ptr_val + i] = 0;
        }

        Ok(())
    }

    pub fn _memset(&mut self) -> Result<()> {
        let ptr = self.next_8()?;

        if ptr > 31 {
            return Err(VmError::InvalidRegister(ptr))
        }

        let value = self.next_8()?;

        if value > 31 {
            return Err(VmError::InvalidRegister(value))
        }

        let amount = self.next_8()?;

        if amount > 31 {
            return Err(VmError::InvalidRegister(amount))
        }

        let ptr_val = self.registers[ptr as usize] as usize;

        if ptr_val >= self.heap.len() {
            return Err(
                VmError::SegmentationFault
            )
        }

        let value_val = self.registers[value as usize];
        let amount_val = self.registers[amount as usize] as usize;

        for i in 0..amount_val {

            if ptr_val + i >= self.heap.len() {
                return Err(
                    VmError::SegmentationFault
                )
            }

            self.heap[ptr_val + i] = if value_val > 255 {
                255
            } else if value_val < 0 {
                0
            } else {
                value_val as u8
            };
        }

        Ok(())
    }

    pub fn _memmove(&mut self) -> Result<()> {

        let end = self.next_8()?;

        if end > 31 {
            return Err(VmError::InvalidRegister(end))
        }

        let start = self.next_8()?;

        if start > 31 {
            return Err(VmError::InvalidRegister(start))
        }

        let amount_reg = self.next_8()?;

        if amount_reg > 31 {
            return Err(
                VmError::InvalidRegister(amount_reg)
            )
        }

        let amount = self.registers[amount_reg as usize] as usize;

        for i in 0..amount {

            let ptr_from = self.registers[start as usize] as usize + i;
            let ptr_to   = self.registers[end as usize] as usize + i;

            if ptr_from >= self.heap.len() || ptr_to >= self.heap.len() {
                return Err(
                    VmError::SegmentationFault
                )
            }
            
            self.heap[ptr_to] = self.heap[ptr_from];
        }

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
