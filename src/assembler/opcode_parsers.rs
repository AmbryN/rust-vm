use nom::{bytes::complete::tag_no_case, character::complete::alpha1, IResult};

use crate::{assembler::Token, instruction::Opcode};

pub fn opcode_parser(input: &str) -> IResult<&str, Token> {
    let (input, token) = alpha1(input)?;
    let token = Token::Op {
        code: Opcode::from(token),
    };

    Ok((input, token))
}

mod tests {
    use super::*;

    #[test]
    fn test_opcode_parse() {
        let result = opcode_parser("load");
        assert!(result.is_ok());
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");

        let result = opcode_parser("LOAD");
        assert!(result.is_ok());
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, "");

        let result = opcode_parser("aold");
        assert!(result.is_ok());
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
        assert_eq!(rest, "");
    }
}
