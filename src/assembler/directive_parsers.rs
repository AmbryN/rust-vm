use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0},
    combinator::opt,
    IResult,
};

use super::{
    instruction_parsers::AssemblerInstruction, label_parsers::label_declaration_parser,
    operand_parsers::operand_parser, Token,
};

pub fn directive_parser(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, label) = opt(label_declaration_parser)(input)?;
    let (input, directive_name) = directive_declaration_parser(input)?;
    let (input, operand1) = opt(operand_parser)(input)?;
    let (input, operand2) = opt(operand_parser)(input)?;
    let (input, operand3) = opt(operand_parser)(input)?;

    Ok((
        input,
        AssemblerInstruction {
            opcode: None,
            directive: Some(directive_name),
            label,
            operand1,
            operand2,
            operand3,
        },
    ))
}

fn directive_declaration_parser(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag(".")(input)?;
    let (input, directive) = alphanumeric1(input)?;
    let (input, _) = multispace0(input)?;

    Ok((
        input,
        Token::Directive {
            name: directive.to_string(),
        },
    ))
}

mod tests {

    use super::*;

    #[test]
    fn test_directive_declaration_parser() {
        let result = directive_declaration_parser(".test ");
        assert!(result.is_ok());
        let (rest, directive) = result.unwrap();
        assert_eq!(
            directive,
            Token::Directive {
                name: "test".to_string()
            }
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn test_directive_parser() {
        let result = directive_parser("test: .asciiz 'Hello'");
        assert!(result.is_ok());
        let (rest, directive) = result.unwrap();
        assert_eq!(
            directive,
            AssemblerInstruction {
                label: Some(Token::LabelDeclaration {
                    name: "test".to_string()
                }),
                directive: Some(Token::Directive {
                    name: "asciiz".to_string()
                }),
                opcode: None,
                operand1: Some(Token::String {
                    value: "Hello".to_string()
                }),
                operand2: None,
                operand3: None,
            }
        )
    }
}
