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
    }
}