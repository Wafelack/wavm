mod vm;
mod compiler;
mod tests;

use vm::{Vm};
use compiler::Compiler;

use std::{fmt::{self, Debug, Formatter}, fs::{self, File}, io::{self, Read, Write}};

use clap::{App, Arg, SubCommand};


pub enum CustomError {
    VmError(vm::VmError),
    CompilerError(compiler::CompilerErrors),
    IoError(io::Error),
}

impl From<io::Error> for CustomError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<vm::VmError> for CustomError {
    fn from(e: vm::VmError) -> Self {
        Self::VmError(e)
    }
}

impl From<compiler::CompilerErrors> for CustomError {
    fn from(e: compiler::CompilerErrors) -> Self {
        Self::CompilerError(e)
    }
}

impl Debug for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        
        match self {
            Self::VmError(e) => write!(f, "A Virtual Machine error(s) occured: {:?}.", e),
            Self::CompilerError(e) => write!(f, "One or more compiler error(s) occured: {:?}.", e),
            Self::IoError(e) => write!(f, "An I/O error occured: {}.", e),
        }
    }
}

fn main() -> std::result::Result<(), CustomError> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                            .version(env!("CARGO_PKG_VERSION"))
                            .author(env!("CARGO_PKG_AUTHORS"))
                            .about("Wait, another Virtual Machine ?")
                            .subcommand(SubCommand::with_name("build")
                                .about("Compiles a .wavm source file into .wavc bytecode.")
                                .arg(Arg::with_name("file")
                            .required(true)
                            .index(1)
                            .takes_value(true))
                                .arg(Arg::with_name("output")
                                    .help("The output file.")
                                    .takes_value(true)
                                    .short("o")))
                            .subcommand(SubCommand::with_name("run")
                                    .arg(Arg::with_name("file")
                                        .index(1)
                                        .required(true)
                                    .takes_value(true))
                                    .about("Runs a .wasc bytecode file"))
                            .get_matches();

    if let Some(m) = matches.subcommand_matches("build") {
        let program = Compiler::new(fs::read_to_string(m.value_of("file").unwrap())?).compile()?;
        let mut f = File::create(if m.is_present("output") {
            m.value_of("output").unwrap().to_owned()
        } else {
            m.value_of("file").unwrap().replace(".wavm", ".wavc")
        })?;

        f.write_all(&program)?;
    } else if let Some(m) = matches.subcommand_matches("run") {
        let mut buffer = vec![];

        File::open(m.value_of("file").unwrap())?.read_to_end(&mut buffer)?;

        let mut vm = Vm::new(buffer);
        vm.run()?;
    }

    Ok(())
}
