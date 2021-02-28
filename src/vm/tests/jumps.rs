use super::*;

#[cfg(test)]
mod test {
    use super::*;

    mod jumps {
        use super::*;

        #[test]
        fn jmp() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, JMP, 0x00, 0x07, HLT, MOVE, 0x01, 0x00,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.get_registers()[1], 10);

            Ok(())
        }

        #[test]
        fn jmpeq() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x02, 0x00, 0x09, LE, 0x02, 0x00, JMPEQ, 0x00, 0x0E,
                HLT, MOVE, 0x01, 0x00,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.get_registers()[1], 10);

            Ok(())
        }

        #[test]
        fn rjmp() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, RJMP, 0x00, 0x01, HLT, MOVE, 0x01, 0x00,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.get_registers()[1], 10);

            Ok(())
        }
    }
}
