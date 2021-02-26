#[cfg(test)]
mod test {
    mod language {
        use crate::vm::*;
        use crate::compiler::{Compiler, CompilerErrors};

        #[test]
        fn load() -> std::result::Result<(), CompilerErrors> {

            let compiled = Compiler::new("load 0 44").compile()?;

            assert_eq!(
                compiled,
                vec![LOAD, 0x0, 0x00, 0x2C]
            );

            Ok(())
        }

        #[test]
        fn bi_reg() -> std::result::Result<(), CompilerErrors> {

            let compiled = Compiler::new("xor 0 0").compile()?;

            assert_eq!(
                compiled,
                vec![XOR, 0x00, 0x00]
            );

            Ok(())
        }

        #[test]
        fn bi_sixteen() -> std::result::Result<(), CompilerErrors> {

            let compiled = Compiler::new("rqm 0 12").compile()?;

            assert_eq!(
                compiled,
                vec![RQM, 0x00, 0x00, 0x0C]
            );

            Ok(())
        }

        #[test]
        fn tri_reg() -> std::result::Result<(), CompilerErrors> {

            let compiled = Compiler::new("mset 0 15 22").compile()?;

            assert_eq!(
                compiled,
                vec![MSET, 0x00, 0x0F, 0x16]
            );

            Ok(())
        }

        #[test]
        fn labels() -> std::result::Result<(), CompilerErrors> {

            let compiled = Compiler::new("\
            :foo
            jmp :foo").compile()?;

            assert_eq!(
                compiled,
                vec![JMP, 0x00, 0x00]
            );

            Ok(())
        }

        #[test]
        fn ascii() -> std::result::Result<(), CompilerErrors> {

            let compiled = Compiler::new("\
            ascii 0 'Hello, World !'").compile()?;

            assert_eq!(
                compiled,
                vec![ASCII, 0, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 32, 33, 0]
            );

            Ok(())
        }        

        #[test]
        fn dsp() -> std::result::Result<(), CompilerErrors> {
            let compiled = Compiler::new("\
            ascii 0 'Hello, World !'
            dsp 0").compile()?;

            assert_eq!(
                compiled,
                vec![ASCII, 0, 72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 32, 33, 0, DSP, 0]
            );

            let mut vm = Vm::new(compiled);
            vm.run().unwrap();

            Ok(())         
        }
        
    }
}