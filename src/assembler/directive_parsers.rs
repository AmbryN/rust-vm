use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0},
    combinator::opt,
    IResult,
};

use super::{instruction_parsers::AssemblerInstruction, operand_parsers::operand_parser, Token};

pub fn directive_parser(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, directive) = directive_declaration_parser(input)?;
    let (input, operand1) = opt(operand_parser)(input)?;
    let (input, operand2) = opt(operand_parser)(input)?;
    let (input, operand3) = opt(operand_parser)(input)?;

    Ok((
        input,
        AssemblerInstruction {
            opcode: None,
            directive: Some(directive),
            label: None,
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
    fn test_label_declaration_parser() {
        let result = directive_declaration_parser(".test ");
        assert!(result.is_ok());
        let (rest, label) = result.unwrap();
        assert_eq!(
            label,
            Token::Directive {
                name: "test".to_string()
            }
        );
        assert_eq!(rest, "");
    }
}
