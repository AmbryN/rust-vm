use nom::{multi::many1, IResult};

use crate::assembler::instruction_parsers::{instruction_parser, AssemblerInstruction};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}

pub fn program_parser(input: &str) -> IResult<&str, Program> {
    let (input, instructions) = many1(instruction_parser)(input)?;

    Ok((input, Program { instructions }))
}

mod tests {

    use super::*;

    #[test]
    fn test_parse_program() {
        let result = program_parser("LOAD $0 #10\nLOAD $1 #15\n");
        assert!(result.is_ok());

        let (input, program) = result.unwrap();
        assert_eq!(input, "");
        assert_eq!(program.instructions.len(), 2);
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program_parser("load $0 #100\n");
        assert_eq!(result.is_ok(), true);
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }
}
