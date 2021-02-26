use super::*;

#[cfg(test)]
mod test {
    pub use super::*;

    mod boolean {
        use super::*;

        #[test]
        fn eq() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, EQ, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert!(vm.get_registers()[31] == 0);

            Ok(())
        }
        #[test]
        fn neq() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, NEQ, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert!(vm.get_registers()[31] == 1);

            Ok(())
        }

        #[test]
        fn geq() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, GEQ, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert!(vm.get_registers()[31] == 1);

            Ok(())
        }

        #[test]
        fn ge() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, GE, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert!(vm.get_registers()[31] == 1);

            Ok(())
        }

        #[test]
        fn leq() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, LEQ, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert!(vm.get_registers()[31] == 0);

            Ok(())
        }

        #[test]
        fn le() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, LE, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert!(vm.get_registers()[31] == 0);

            Ok(())
        }

        #[test]
        fn not() -> Result<()> {
            let program = vec![LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, NOT, 0x00];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert!(vm.get_registers()[31] == 0);

            Ok(())
        }

        #[test]
        fn and() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, AND, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert!(vm.get_registers()[31] == 1);

            Ok(())
        }

        #[test]
        fn or() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, OR, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert!(vm.get_registers()[31] == 1);

            Ok(())
        }

        #[test]
        fn xor() -> Result<()> {
            let program = vec![
                LOAD, 0x00, 0x00, 0x0A, LOAD, 0x01, 0x00, 0x02, XOR, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert!(vm.get_registers()[31] == 0);

            Ok(())
        }
    }
}