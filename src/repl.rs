use crate::assembler::program_parsers::program_parser;
use crate::vm::VM;
use std;
use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::path::Path;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to the VM! Let's be productive!");
        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".program" => {
                    println!("Listing instructions currentli in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");
                }
                ".register" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing")
                }
                ".load_file" => {
                    print!("Please enter the path to the file you wish to load: ");
                    io::stdout().flush().expect("Unable to flush stdout");
                    let mut tmp = String::new();
                    stdin
                        .read_line(&mut tmp)
                        .expect("Unable to read line from user");
                    let tmp = tmp.trim();
                    // let filename = Path::new(&tmp);
                    let mut f = File::open(Path::new(&tmp)).expect("File not found");
                    let mut contents = String::new();
                    f.read_to_string(&mut contents)
                        .expect("There was an error reading from the file");
                    let program = match program_parser(&contents) {
                        Ok((_, program)) => program,
                        Err(e) => {
                            println!("Unable to parse input: {:?}", e);
                            continue;
                        }
                    };
                    self.vm.program.append(&mut program.to_bytes());
                }
                ".clear" => {
                    self.vm.program.clear();
                }
                ".quit" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                _ => {
                    let program = match program_parser(buffer) {
                        Ok((_, program)) => program,
                        Err(_) => {
                            println!("Unable to parse input");
                            continue;
                        }
                    };
                    self.vm.program.append(&mut program.to_bytes());
                    self.vm.run_once();
                }
            }
        }
    }

    #[allow(dead_code)]
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let bytes = i.split(' ');
        let results: Result<Vec<u8>, ParseIntError> = bytes
            .into_iter()
            .map(|byte| u8::from_str_radix(byte, 16))
            .collect();
        results
    }
}

impl Default for REPL {
    fn default() -> Self {
        Self::new()
    }
}
