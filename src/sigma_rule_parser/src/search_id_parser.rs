use nom::branch::alt;
use nom::bytes::complete::{take_until, take_while};
use nom::combinator::{rest, value};
use nom::IResult;

// use log_analyzer::detection::{Condition, OPERATOR, PARSER_TYPES};

use crate::parser_output::ParserOutput;
use crate::structs::condition::{Condition, PARSER_TYPES};

/// TODO: Support wild card names - handled in Detection creation?
/// Returns search identifiers within a condition (take_until(" ")), and at the end of a condition (rest of string)
/// Returns the remaining string to parse, the result that was parsed, and the condition being updated.
/// A successful response indicates that the condition is completed and ready to be stored in a Detection struct
/// A failure indicates invalid input, or potentially a missed parsing use-case.
fn search_identifiers(
    input: &str
) -> IResult<&str, &str> {
    let sid = alt((take_until(" "), rest))(input.trim());

    let sid_result = match sid {
        Ok(parsed_sid) => parsed_sid,
        Err(e) => return Err(e),
    };

    Ok(sid_result)
}

pub fn search_identifiers_parser(
    input: &str
) -> IResult<&str, ParserOutput<Condition>> {
    let mut condition = Condition::new();
    let (remaining, result) = search_identifiers(input)?;

    condition.parser_type = Some(PARSER_TYPES::SEARCH_IDENTIFIER);
    condition.parser_result = Some(vec![result.to_string()]);
    condition.search_identifier = Some(result.to_string());
    value(ParserOutput { input: { condition.clone() } }, take_while(|ch| ch != ' '))(input.trim())
}

fn parser_str_builder(input: Option<Vec<String>>) -> String {
    input.as_ref().unwrap().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_identifier_condition() {

        let result = search_identifiers_parser("Selection");
        assert_eq!(result, Ok((
            "",
            ParserOutput {
                input: Condition {
                    parser_type: Some(PARSER_TYPES::SEARCH_IDENTIFIER),
                    parser_result: Some(vec!["Selection".to_string()]),
                    is_negated: None,
                    operator: None,
                    search_identifier: Some("Selection".to_string()),
                    nested_detections: None
                }
            }
        )));

        let result = search_identifiers_parser("Selection and not Filter");
        assert_eq!(result, Ok((
            " and not Filter",
            ParserOutput {
                input: Condition {
                    parser_type: Some(PARSER_TYPES::SEARCH_IDENTIFIER),
                    parser_result: Some(vec!["Selection".to_string()]),
                    is_negated: None,
                    operator: None,
                    search_identifier: Some("Selection".to_string()),
                    nested_detections: None
                }
            }
        )));

        let result = search_identifiers_parser("");
        assert_eq!(result, Ok((
            "",
            ParserOutput {
                input: Condition {
                    parser_type: Some(PARSER_TYPES::SEARCH_IDENTIFIER),
                    parser_result: Some(vec!["".to_string()]),
                    is_negated: None,
                    operator: None,
                    search_identifier: Some("".to_string()),
                    nested_detections: None
                }
            }
        )));
    }

    #[test]
    fn search_identifier_input() {
        let mid_condition_parser_result = search_identifiers(" Selection and not Filter ");
        assert_eq!(
            mid_condition_parser_result,
            Ok((" and not Filter", "Selection"))
        );

        let end_of_condition_parser_result = search_identifiers(" Events ");
        assert_eq!(end_of_condition_parser_result, Ok(("", "Events")));

        let empty_string_parser_result = search_identifiers("");
        assert_eq!(
            empty_string_parser_result,
            Ok(("", ""))
        );

        let empty_string_parser_result = search_identifiers("    ");
        assert_eq!(empty_string_parser_result, Ok(("", "")));
    }
}
