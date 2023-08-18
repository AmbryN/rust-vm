mod directive_parsers;
mod instruction_parsers;
mod label_parsers;
mod opcode_parsers;
mod operand_parsers;
pub mod program_parsers;
mod register_parsers;
mod symbol;

use crate::instruction::Opcode;

use self::{
    program_parsers::{program_parser, Program},
    symbol::{Symbol, SymbolTable, SymbolType},
};

pub const PIE_HEADER_PREFIX: [u8; 4] = [45, 50, 49, 45];
pub const PIE_HEADER_LENGTH: usize = 64;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}

#[derive(Debug)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
        }
    }

    fn write_pie_header(&self) -> Vec<u8> {
        let mut header = Vec::with_capacity(PIE_HEADER_LENGTH);
        header.append(&mut PIE_HEADER_PREFIX.to_vec());
        header.fill(0);
        header
    }

    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        match program_parser(raw) {
            Ok((_remainder, program)) => {
                let mut assembled_program = self.write_pie_header();

                self.process_first_phase(&program);
                let mut body = self.process_second_phase(&program);

                assembled_program.append(&mut body);
                Some(assembled_program)
            }
            Err(e) => {
                println!("There was an error assembling the code: {:?}", e);
                None
            }
        }
    }

    fn process_first_phase(&mut self, p: &Program) {
        self.extract_labels(p);
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        let mut program = vec![];
        for i in &p.instructions {
            let mut bytes = i.to_bytes();
            program.append(&mut bytes);
        }
        program
    }

    fn extract_labels(&mut self, p: &Program) {
        let mut c = 0;
        for i in &p.instructions {
            if i.is_label() {
                if let Some(name) = i.label_name() {
                    let symbol = Symbol::new(name, SymbolType::Label, c);
                    self.symbols.add_symbol(symbol);
                }
            }
            c += 4;
        }
    }
}

#[derive(Debug)]
pub enum AssemblerPhase {
    First,
    Second,
}
