mod directive_parsers;
mod instruction_parsers;
mod label_parsers;
mod opcode_parsers;
mod operand_parsers;
pub mod program_parsers;
mod register_parsers;
mod symbol;

use nom::error::Error;

use crate::instruction::Opcode;

use self::{
    instruction_parsers::AssemblerInstruction,
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
    String { value: String },
}

#[derive(Debug)]
pub struct Assembler {
    /// Symbol table for constants and variables
    pub symbols: SymbolTable,
    /// The read-only data section constants are put in
    pub ro: Vec<u8>,
    /// The compiled bytecode generated from the assembly instructions
    pub bytecode: Vec<u8>,
    /// Tracks the current offset of the read-only section
    ro_offset: u32,
    /// A list of all the sections we've seen in the code
    sections: Vec<AssemblerSection>,
    /// The current section the assembler is in
    current_section: Option<AssemblerSection>,
    /// The current instruction the assembler is converting to bytecode
    current_instruction: u32,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            symbols: SymbolTable::new(),
            sections: vec![],
            ro: vec![],
            bytecode: vec![],
            ro_offset: 0,
            current_section: None,
            current_instruction: 0,
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, AssemblerError> {
        let (_, program) = program_parser(raw)?;

        self.extract_sections_and_labels(&program)?;

        let mut assembled_program = self.write_pie_header();
        let mut body = self.process_second_phase(&program)?;

        assembled_program.append(&mut body);

        Ok(assembled_program)
    }

    fn extract_sections_and_labels(&mut self, p: &Program) -> Result<(), AssemblerError> {
        for i in &p.instructions {
            if i.is_section() {
                let section: AssemblerSection =
                    i.get_directive_name().unwrap().as_str().try_into()?;
                self.sections.push(section);
            } else if i.is_label() {
                if self.sections.len() > 0 {
                    if let Some(name) = i.get_label_name() {
                        let symbol = Symbol::new(name, SymbolType::Label, 0);
                        if self.symbols.has_symbol(&symbol) {
                            return Err(AssemblerError::SymbolAlreadyDeclared { symbol });
                        }
                        self.symbols.add_symbol(symbol);
                    } else {
                        return Err(AssemblerError::StringConstantDeclaredWithoutLabel);
                    }
                }
            }
        }
        Ok(())
    }

    fn write_pie_header(&self) -> Vec<u8> {
        let mut header = Vec::with_capacity(PIE_HEADER_LENGTH);
        header.append(&mut PIE_HEADER_PREFIX.to_vec());
        header.fill(0);
        header
    }

    fn process_second_phase(&mut self, p: &Program) -> Result<Vec<u8>, AssemblerError> {
        self.current_instruction = 0;

        let mut program = vec![];
        for i in &p.instructions {
            if i.is_section() {
                continue;
            } else if i.is_opcode() {
                let mut bytes = i.to_bytes();
                program.append(&mut bytes);
            } else if i.is_directive() {
                self.process_directive(i)?;
            }
            self.current_instruction += 1;
        }
        Ok(program)
    }

    fn process_directive(&mut self, i: &AssemblerInstruction) -> Result<(), AssemblerError> {
        if let Some(directive_name) = i.get_directive_name() {
            if i.has_operands() {
                match directive_name.as_str() {
                    "asciiz" => return self.handle_asciiz(i),
                    _ => {
                        return Err(AssemblerError::UnknownDirectiveFound {
                            directive: directive_name,
                        })
                    }
                }
            } else {
                return Err(AssemblerError::DirectiveWithoutOperand {
                    directive: directive_name,
                });
            }
        }
        return Err(AssemblerError::DirectiveHasInvalidName);
    }

    fn handle_asciiz(&mut self, i: &AssemblerInstruction) -> Result<(), AssemblerError> {
        if let Some(s) = i.get_string_constant() {
            if let Some(name) = i.get_label_name() {
                self.symbols.set_symbol_offset(&name, self.ro_offset);
            } else {
                // This would be someone typing:
                // .asciiz 'Hello'
                return Err(AssemblerError::NoLabel { string: s });
            }

            // We'll read the string into the read-only section byte-by-byte
            for byte in s.as_bytes() {
                self.ro.push(*byte);
                self.ro_offset += 1;
            }
            // This is the null termination bit we are using to indicate a string has ended
            self.ro.push(0);
            self.ro_offset += 1;
            Ok(())
        } else {
            // This just means someone typed `.asciiz` for some reason
            return Err(AssemblerError::NoStringConstant);
        }
    }
}

#[derive(Debug)]
pub enum AssemblerError {
    SymbolAlreadyDeclared { symbol: Symbol },
    StringConstantDeclaredWithoutLabel,
    ParseError { error: String },
    InsufficientSections,
    UnknownDirectiveFound { directive: String },
    DirectiveHasInvalidName,
    UnknownSectionFound,
    ShouldBeSecondPhase,
    NoStringConstant,
    NoLabel { string: String },
    DirectiveWithoutOperand { directive: String },
}

impl From<nom::Err<nom::error::Error<&str>>> for AssemblerError {
    fn from(value: nom::Err<Error<&str>>) -> Self {
        AssemblerError::ParseError {
            error: "Test".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum AssemblerSection {
    Header,
    Data,
    Code,
}

impl TryFrom<&str> for AssemblerSection {
    type Error = AssemblerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "header" => Ok(AssemblerSection::Header),
            "data" => Ok(AssemblerSection::Data),
            "code" => Ok(AssemblerSection::Code),
            _ => Err(AssemblerError::UnknownSectionFound),
        }
    }
}
