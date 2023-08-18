use nom::{branch::alt, character::complete::multispace1, IResult};

use crate::assembler::{
    opcode_parsers::opcode_parser, operand_parsers::operand_parser,
    register_parsers::register_parser, Token,
};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Token::Op { code } => {
                results.push(code as u8);
            }
            _ => {
                println!("Non opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            if let Some(t) = operand {
                AssemblerInstruction::extract_operand(t, &mut results);
            }
        }

        results
    }

    fn extract_operand(token: &Token, results: &mut Vec<u8>) {
        match token {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }
}

pub fn instruction_parser(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, instruction) = alt((
        instruction_register_value_parser,
        instruction_three_registers_parser,
        instruction_opcode_only_parser,
    ))(input)?;

    Ok((input, instruction))
}

fn instruction_register_value_parser(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, opcode) = opcode_parser(input)?;
    let (input, _) = multispace1(input)?;
    let (input, register) = register_parser(input)?;
    let (input, _) = multispace1(input)?;
    let (input, operand) = operand_parser(input)?;

    Ok((
        input,
        AssemblerInstruction {
            opcode,
            operand1: Some(register),
            operand2: Some(operand),
            operand3: None,
        },
    ))
}

fn instruction_opcode_only_parser(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, opcode) = opcode_parser(input)?;

    Ok((
        input,
        AssemblerInstruction {
            opcode,
            operand1: None,
            operand2: None,
            operand3: None,
        },
    ))
}

fn instruction_three_registers_parser(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, opcode) = opcode_parser(input)?;
    let (input, _) = multispace1(input)?;
    let (input, register1) = register_parser(input)?;
    let (input, _) = multispace1(input)?;
    let (input, register2) = register_parser(input)?;
    let (input, _) = multispace1(input)?;
    let (input, register3) = register_parser(input)?;

    Ok((
        input,
        AssemblerInstruction {
            opcode,
            operand1: Some(register1),
            operand2: Some(register2),
            operand3: Some(register3),
        },
    ))
}

mod tests {

    use crate::instruction::Opcode;

    use super::*;

    #[test]
    fn test_parse_instruction() {
        let result = instruction_parser("LOAD $0 #10");
        assert!(result.is_ok());

        let (input, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                opcode: Token::Op { code: Opcode::LOAD },
                operand1: Some(Token::Register { reg_num: 0 }),
                operand2: Some(Token::IntegerOperand { value: 10 }),
                operand3: None
            }
        );
        assert_eq!(input, "");

        let result = instruction_parser("ADD $0 $1 $2");
        assert!(result.is_ok());

        let (input, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                opcode: Token::Op { code: Opcode::ADD },
                operand1: Some(Token::Register { reg_num: 0 }),
                operand2: Some(Token::Register { reg_num: 1 }),
                operand3: Some(Token::Register { reg_num: 2 })
            }
        );
        assert_eq!(input, "");

        let result = instruction_parser("HLT");
        assert!(result.is_ok());

        let (input, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                opcode: Token::Op { code: Opcode::HLT },
                operand1: None,
                operand2: None,
                operand3: None
            }
        );
        assert_eq!(input, "");
    }
}
