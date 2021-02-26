use super::*;

#[cfg(test)]
mod test {
    use super::*;
    
    mod arithmetic {
        use super::*;

        #[test]
        fn add() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, ADD, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.get_registers()[0], 12);

            Ok(())
        }
        #[test]
        fn sub() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, SUB, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.get_registers()[0], 8);

            Ok(())
        }

        #[test]
        fn mul() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, MUL, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.get_registers()[0], 20);

            Ok(())
        }

        #[test]
        fn div() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, DIV, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.get_registers()[0], 5);

            Ok(())
        }

        #[test]
        fn mod_() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, MOD, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.get_registers()[0], 0);

            Ok(())
        }
    }
}