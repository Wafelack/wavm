use super::*;

#[cfg(test)]
mod test {
    use super::*;

    mod memory {
        use super::*;

        #[test]
        fn rqm() -> Result<()> {
            let program = vec![RQM, 0x00, 0x00, 0x0F];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.heap.len(), 15,);

            assert_eq!(vm.get_registers()[0], 0);

            Ok(())
        }
        #[test]
        fn ascii() -> Result<()> {
            let program = vec![
                ASCII, 0x00, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 0,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.heap.len(), 13);

            assert_eq!(vm.get_registers()[0], 0);

            assert_eq!(
                vm.heap[..11].iter().map(|u| *u as char).collect::<String>(),
                "Hello World".to_owned()
            );

            Ok(())
        }

        #[test]
        fn set_byte() -> Result<()> {
            let program = vec![
                LOAD, 0x01, 0x00, 0x16, RQM, 0x00, 0x00, 0x01, SETB, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.heap.len(), 1);

            assert_eq!(vm.get_registers()[0], 0);

            assert_eq!(vm.heap[vm.get_registers()[0] as usize], 22);

            Ok(())
        }

        #[test]
        fn get_byte() -> Result<()> {
            let program = vec![
                LOAD, 0x01, 0x00, 0x16, RQM, 0x00, 0x00, 0x01, SETB, 0x00, 0x01, GETB, 0x02, 0x00,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.heap.len(), 1);

            assert_eq!(vm.get_registers()[0], 0);

            assert_eq!(vm.get_registers()[2], 22);

            Ok(())
        }

        #[test]
        fn memmove() -> Result<()> {
            let program = vec![
                LOAD, 0x01, 0x00, 0x01, RQM, 0x00, 0x00, 0x02, SETB, 0x00, 0x01, MMOV, 0x01, 0x00,
                0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.heap.len(), 2);

            assert_eq!(vm.get_registers()[0], 0);

            assert_eq!(vm.heap[1], 1);

            Ok(())
        }

        #[test]
        fn memset() -> Result<()> {
            let program = vec![
                RQM, 0x00, 0x00, 0x05, LOAD, 0x01, 0x00, 0x05, MSET, 0x00, 0x01, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.heap.len(), 5);

            assert_eq!(vm.get_registers()[0], 0);

            for i in 0..5 {
                assert_eq!(vm.heap[i], 5)
            }

            Ok(())
        }

        #[test]
        fn free() -> Result<()> {
            let program = vec![
                LOAD, 0x01, 0x00, 0x0C, ASCII, 0x00, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108,
                100, 0, FREE, 0x00, 0x01,
            ];
            let mut vm = Vm::new(program);
            vm.run()?;

            assert_eq!(vm.heap.len(), 13);

            for i in 0..vm.heap.len() {
                assert_eq!(vm.heap[i], 0);
            }

            Ok(())
        }
    }
}
