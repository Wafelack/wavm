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

        mod arithmetic {
            use super::*;

            #[test]
            fn add() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          ADD, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert_eq!(
                    vm.get_registers()[0],
                    12
                );

                Ok(())
            }
            #[test]
            fn sub() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          SUB, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert_eq!(
                    vm.get_registers()[0],
                    8
                );

                Ok(())
            }

            #[test]
            fn mul() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          MUL, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert_eq!(
                    vm.get_registers()[0],
                    20
                );

                Ok(())
            }

            #[test]
            fn div() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          DIV, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert_eq!(
                    vm.get_registers()[0],
                    5
                );

                Ok(())
            }

            #[test]
            fn mod_() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          MOD, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert_eq!(
                    vm.get_registers()[0],
                    0
                );

                Ok(())
            }
        }

        mod boolean {
            use super::*;

            #[test]
            fn eq() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          EQ, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert!(vm.get_registers()[31] == 0);

                Ok(())
            }
            #[test]
            fn neq() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          NEQ, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert!(vm.get_registers()[31] == 1);

                Ok(())
            }

            #[test]
            fn geq() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          GEQ, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert!(vm.get_registers()[31] == 1);

                Ok(())
            }

            #[test]
            fn ge() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          GE, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert!(vm.get_registers()[31] == 1);

                Ok(())
            }

            #[test]
            fn leq() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          LEQ, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert!(vm.get_registers()[31] == 0);

                Ok(())
            }

            #[test]
            fn le() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          LE, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert!(vm.get_registers()[31] == 0);

                Ok(())
            }

            #[test]
            fn not() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          NOT, 0x00];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert!(vm.get_registers()[31] == 0);

                Ok(())
            }

            #[test]
            fn and() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          AND, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert!(vm.get_registers()[31] == 1);

                Ok(())
            }

            #[test]
            fn or() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          OR, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert!(vm.get_registers()[31] == 1);

                Ok(())
            }

            #[test]
            fn xor() -> Result<()> {
                let program = vec![LOAD, 0x00, 0x00, 0x0A,
                                          LOAD, 0x01, 0x00, 0x02,
                                          XOR, 0x00, 0x01];
                let mut vm = Vm::new(program);
                vm.run()?;

                assert!(vm.get_registers()[31] == 0);

                Ok(())
            }
        }
    }
}