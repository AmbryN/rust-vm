use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{digit1, multispace0},
    IResult,
};

use crate::assembler::Token;

use super::register_parsers::register_parser;

pub fn operand_parser(input: &str) -> IResult<&str, Token> {
    let (input, operand) = alt((register_parser, value_parser, string_parser))(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, operand))
}

fn value_parser(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("#")(input)?;
    let (input, operand) = digit1(input)?;

    Ok((
        input,
        Token::IntegerOperand {
            value: operand.parse::<i32>().unwrap(),
        },
    ))
}

fn string_parser(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("'")(input)?;
    let (input, string) = take_until1("'")(input)?;
    let (input, _) = tag("'")(input)?;

    Ok((
        input,
        Token::String {
            value: string.to_string(),
        },
    ))
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_operand() {
        let result = value_parser("#10");
        assert!(result.is_ok());
        let (rest, value) = result.unwrap();
        assert_eq!(value, Token::IntegerOperand { value: 10 });
        assert_eq!(rest, "");

        let result = value_parser("10");
        assert!(!result.is_ok());

        let result = value_parser("#");
        assert!(!result.is_ok());
    }

    #[test]
    fn test_parse_string() {
        let result = string_parser("'hello'");
        assert!(result.is_ok());
        let (rest, value) = result.unwrap();
        assert_eq!(
            value,
            Token::String {
                value: "hello".to_string()
            }
        );
        assert_eq!(rest, "");
    }
}
