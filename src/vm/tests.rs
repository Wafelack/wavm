pub use super::*;

#[cfg(test)]
mod test {
    pub use super::*;

    mod opcodes {
        use super::*;

        #[test]
        fn load() -> Result<()> {
            let program = vec![LOAD, 0x00, 0x00, 0x1A];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(
                vm.get_registers()[0],
                26
            );


            Ok(())
        }

        #[test]
        fn move_() -> Result<()> {
            let program = vec![LOAD, 0x00, 0x00, 0x1A,
                                       MOVE, 0x01, 0x00];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(
                vm.get_registers()[1],
                vm.get_registers()[0],
            );


            Ok(())
        }

        #[test]
        fn halt() -> Result<()> {
            let program = vec![LOAD, 0x00, 0x00, 0x1A,
                                       HLT,
                                       MOVE, 0x01, 0x00];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(
                vm.get_registers()[1],
                0
            );


            Ok(())
        }
    }
}