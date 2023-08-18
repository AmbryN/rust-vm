use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0},
    IResult,
};

use super::Token;

pub fn label_declaration_parser(input: &str) -> IResult<&str, Token> {
    let (input, label) = alphanumeric1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;

    Ok((
        input,
        Token::LabelDeclaration {
            name: label.to_string(),
        },
    ))
}

pub fn label_usage_parser(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag("@")(input)?;
    let (input, label) = alphanumeric1(input)?;
    let (input, _) = multispace0(input)?;

    Ok((
        input,
        Token::LabelUsage {
            name: label.to_string(),
        },
    ))
}

mod tests {

    use super::*;

    #[test]
    fn test_label_declaration_parser() {
        let result = label_declaration_parser("test: ");
        assert!(result.is_ok());
        let (rest, label) = result.unwrap();
        assert_eq!(
            label,
            Token::LabelDeclaration {
                name: "test".to_string()
            }
        );
        assert_eq!(rest, "");
    }

    #[test]
    fn test_label_usage_parser() {
        let result = label_usage_parser("@test ");
        assert!(result.is_ok());
        let (rest, label) = result.unwrap();
        assert_eq!(
            label,
            Token::LabelUsage {
                name: "test".to_string()
            }
        );
        assert_eq!(rest, "");
    }
}
