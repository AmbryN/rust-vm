use nom::{branch::alt, character::complete::newline, combinator::opt, IResult};

use crate::assembler::{opcode_parsers::opcode_parser, operand_parsers::operand_parser, Token};

use super::{
    directive_parsers::directive_parser, label_parsers::label_declaration_parser,
    symbol::SymbolTable,
};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];
        match self.opcode {
            Some(Token::Op { code }) => {
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
        while results.len() < 4 {
            results.push(0);
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

    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn is_opcode(&self) -> bool {
        self.opcode.is_some()
    }

    pub fn is_directive(&self) -> bool {
        self.directive.is_some()
    }

    pub fn get_directive_name(&self) -> Option<String> {
        match &self.directive {
            Some(Token::Directive { name }) => Some(name.clone()),
            _ => None,
        }
    }

    pub fn has_operands(&self) -> bool {
        self.operand1.is_some()
    }

    pub fn get_label_name(&self) -> Option<String> {
        if let Some(Token::LabelDeclaration { name }) = &self.label {
            Some(name.to_owned())
        } else {
            None
        }
    }

    pub fn get_string_constant(&self) -> Option<String> {
        match &self.operand1 {
            Some(Token::String { value }) => Some(value.clone()),
            _ => None,
        }
    }
}

pub fn instruction_parser(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, instruction) = alt((instruction_combined, directive_parser))(input)?;
    let (input, _) = opt(newline)(input)?;

    Ok((input, instruction))
}

fn instruction_combined(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, label) = opt(label_declaration_parser)(input)?;
    let (input, opcode) = opcode_parser(input)?;
    let (input, operand1) = opt(operand_parser)(input)?;
    let (input, operand2) = opt(operand_parser)(input)?;
    let (input, operand3) = opt(operand_parser)(input)?;

    Ok((
        input,
        AssemblerInstruction {
            label,
            opcode: Some(opcode),
            directive: None,
            operand1,
            operand2,
            operand3,
        },
    ))
}

mod tests {

    use crate::instruction::Opcode;

    use super::*;

    #[test]
    fn test_parse_instruction_combined() {
        let result = instruction_combined("LOAD $0 #10");
        assert!(result.is_ok());

        let (input, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                directive: None,
                label: None,
                opcode: Some(Token::Op { code: Opcode::LOAD }),
                operand1: Some(Token::Register { reg_num: 0 }),
                operand2: Some(Token::IntegerOperand { value: 10 }),
                operand3: None,
            }
        );
        assert_eq!(input, "");

        let result = instruction_combined("ADD $0 $1 $2");
        assert!(result.is_ok());

        let (input, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                directive: None,
                label: None,
                opcode: Some(Token::Op { code: Opcode::ADD }),
                operand1: Some(Token::Register { reg_num: 0 }),
                operand2: Some(Token::Register { reg_num: 1 }),
                operand3: Some(Token::Register { reg_num: 2 }),
            }
        );
        assert_eq!(input, "");

        let result = instruction_combined("test: HLT");
        assert!(result.is_ok());

        let (input, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                directive: None,
                label: Some(Token::LabelDeclaration {
                    name: "test".to_string()
                }),
                opcode: Some(Token::Op { code: Opcode::HLT }),
                operand1: None,
                operand2: None,
                operand3: None,
            }
        );
        assert_eq!(input, "");

        let result = instruction_combined("test: LOAD $0 #10");
        assert!(result.is_ok());

        let (input, instruction) = result.unwrap();
        assert_eq!(
            instruction,
            AssemblerInstruction {
                directive: None,
                label: Some(Token::LabelDeclaration {
                    name: "test".to_string()
                }),
                opcode: Some(Token::Op { code: Opcode::LOAD }),
                operand1: Some(Token::Register { reg_num: 0 }),
                operand2: Some(Token::IntegerOperand { value: 10 }),
                operand3: None,
            }
        );
        assert_eq!(input, "");
    }
}
