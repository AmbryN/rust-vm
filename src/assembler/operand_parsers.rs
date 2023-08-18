
use nom::{bytes::complete::tag, character::complete::digit1, IResult};

use crate::assembler::Token;

pub fn operand_parser(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("#")(input)?;
    let (input, operand) = digit1(input)?;

    Ok((
        input,
        Token::IntegerOperand {
            value: operand.parse::<i32>().unwrap(),
        },
    ))
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_operand() {
        let result = operand_parser("#10");
        assert!(result.is_ok());
        let (rest, value) = result.unwrap();
        assert_eq!(value, Token::IntegerOperand { value: 10 });
        assert_eq!(rest, "");

        let result = operand_parser("10");
        assert!(!result.is_ok());

        let result = operand_parser("#");
        assert!(!result.is_ok());
    }
}
