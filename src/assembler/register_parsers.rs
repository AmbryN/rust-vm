
use nom::{bytes::complete::tag, character::complete::digit1, IResult};

use crate::assembler::Token;

pub fn register_parser(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("$")(input)?;
    let (input, register) = digit1(input)?;

    Ok((
        input,
        Token::Register {
            reg_num: register.parse::<u8>().unwrap(),
        },
    ))
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register_parser("$0");
        assert!(result.is_ok());
        let (rest, register) = result.unwrap();
        assert_eq!(register, Token::Register { reg_num: 0 });
        assert_eq!(rest, "");

        let result = register_parser("$10");
        assert!(result.is_ok());
        let (rest, register) = result.unwrap();
        assert_eq!(register, Token::Register { reg_num: 10 });
        assert_eq!(rest, "");

        let result = register_parser("0");
        assert!(!result.is_ok());

        let result = register_parser("0");
        assert!(!result.is_ok());
    }
}
