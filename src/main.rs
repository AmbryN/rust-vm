pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;
use std::{fs::File, io::Read, path::Path};

use clap::{Arg, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file to run
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.file {
        Some(file) => {
            let program = read_file(&file);
            let mut asm = assembler::Assembler::new();
            let mut vm = vm::VM::new();
            let program = asm.assemble(&program);
            if let Some(p) = program {
                vm.add_bytes(p);
                vm.run();
                std::process::exit(0);
            }
        }
        None => {
            start_repl();
        }
    }
}

fn start_repl() {
    let mut repl = repl::REPL::new();
    repl.run();
}

fn read_file(tmp: &str) -> String {
    let mut contents = String::new();
    File::open(Path::new(&tmp))
        .and_then(|mut fh| fh.read_to_string(&mut contents))
        .unwrap();

    contents
}
