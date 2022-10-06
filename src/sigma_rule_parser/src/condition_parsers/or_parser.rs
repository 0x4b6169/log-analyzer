use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::value;
use nom::IResult;

use crate::structs::detection_condition::{DetectionCondition, Metadata, Operator, ParserTypes};
use crate::condition_parsers::not_parser::not_parser;
use crate::condition_parsers::parens_parser::parens_parser;
use crate::condition_parsers::parser_output::ParserOutput;
use crate::condition_parsers::search_id_parser::search_identifiers_parser;

pub fn or_parser(input: &str) -> IResult<&str, ParserOutput<DetectionCondition>> {
    let mut condition = DetectionCondition::init();
    let (remaining, result) = or(input)?;
    let mut result_condition: String = String::from(result);

    let or_parser_result = downstream_or_parser(remaining.trim());
    match or_parser_result {
        Ok((_, parser_output)) => {
            let downstream_parser_result = parser_output.metadata.parser_result.clone();
            result_condition = format!("{}{}{}", result_condition, " ", downstream_parser_result);

            let metadata = Metadata::new(ParserTypes::Or, result_condition.clone());

            condition = DetectionCondition::new(
                metadata,
                parser_output.is_negated.clone(),
                Some(Operator::Or),
                parser_output.search_identifier.clone(),
                parser_output.nested_detections.clone(),
            );
        }
        Err(err) => println!("{:?}", err),
    }

    value(
        ParserOutput {
            result: { condition.clone() },
        },
        tag_no_case(result_condition.clone().as_str()),
    )(input.trim())
}

fn or(input: &str) -> IResult<&str, &str> {
    tag_no_case("or")(input.trim())
}

pub fn downstream_or_parser(input: &str) -> IResult<&str, ParserOutput<DetectionCondition>> {
    alt((parens_parser, not_parser, search_identifiers_parser))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::detection::Detection;
    use nom::error::ErrorKind::Tag;
    use nom::error::{Error, ParseError};

    #[test]
    fn or_parens_parser_condition_with_remaining() {
        let result = or_parser("or (filter and not selection) or keywords");
        assert_eq!(
            result,
            Ok((
                " or keywords",
                ParserOutput {
                    result: DetectionCondition {
                        metadata: Metadata {
                            parser_type: ParserTypes::Or,
                            parser_result: "or (filter and not selection)".to_string(),
                        },
                        is_negated: None,
                        operator: Some(Operator::Or),
                        search_identifier: None,
                        nested_detections: Some(Detection {
                            operator: Some(Operator::And),
                            conditions: Some(vec![
                                DetectionCondition {
                                    metadata: Metadata {
                                        parser_type: ParserTypes::SearchIdentifier,
                                        parser_result: "filter".to_string()
                                    },
                                    is_negated: None,
                                    operator: None,
                                    search_identifier: Some("filter".to_string()),
                                    nested_detections: None
                                },
                                DetectionCondition {
                                    metadata: Metadata {
                                        parser_type: ParserTypes::And,
                                        parser_result: "and not selection".to_string()
                                    },
                                    is_negated: Some(true),
                                    operator: Some(Operator::And),
                                    search_identifier: Some("selection".to_string()),
                                    nested_detections: None
                                }
                            ])
                        })
                    }
                }
            ))
        );
    }

    #[test]
    fn or_parens_parser_condition() {
        let result = or_parser("or (filter and not selection)");
        assert_eq!(
            result,
            Ok((
                "",
                ParserOutput {
                    result: DetectionCondition {
                        metadata: Metadata {
                            parser_type: ParserTypes::Or,
                            parser_result: "or (filter and not selection)".to_string(),
                        },
                        is_negated: None,
                        operator: Some(Operator::Or),
                        search_identifier: None,
                        nested_detections: Some(Detection {
                            operator: Some(Operator::And),
                            conditions: Some(vec![
                                DetectionCondition {
                                    metadata: Metadata {
                                        parser_type: ParserTypes::SearchIdentifier,
                                        parser_result: "filter".to_string()
                                    },
                                    is_negated: None,
                                    operator: None,
                                    search_identifier: Some("filter".to_string()),
                                    nested_detections: None
                                },
                                DetectionCondition {
                                    metadata: Metadata {
                                        parser_type: ParserTypes::And,
                                        parser_result: "and not selection".to_string()
                                    },
                                    is_negated: Some(true),
                                    operator: Some(Operator::And),
                                    search_identifier: Some("selection".to_string()),
                                    nested_detections: None
                                }
                            ])
                        })
                    }
                }
            ))
        );
    }

    #[test]
    fn or_not_parser_condition() {
        let result = or_parser("or not filter");
        assert_eq!(
            result,
            Ok((
                "",
                ParserOutput {
                    result: DetectionCondition {
                        metadata: Metadata {
                            parser_type: ParserTypes::Or,
                            parser_result: "or not filter".to_string(),
                        },
                        is_negated: Some(true),
                        operator: Some(Operator::Or),
                        search_identifier: Some("filter".to_string()),
                        nested_detections: None
                    }
                }
            ))
        );
    }

    #[test]
    fn or_parser_condition() {
        let result = or_parser("or filter");
        assert_eq!(
            result,
            Ok((
                "",
                ParserOutput {
                    result: DetectionCondition {
                        metadata: Metadata {
                            parser_type: ParserTypes::Or,
                            parser_result: "or filter".to_string(),
                        },
                        is_negated: None,
                        operator: Some(Operator::Or),
                        search_identifier: Some("filter".to_string()),
                        nested_detections: None
                    }
                }
            ))
        );
    }

    #[test]
    fn or_parens_input() {
        let parser_result = or(" or (events and selection) ");
        assert_eq!(parser_result, Ok((" (events and selection)", "or")));
    }

    #[test]
    fn or_input() {
        let parser_result = or(" or events ");
        assert_eq!(parser_result, Ok((" events", "or")));

        let parser_result = or(" and events ");
        assert_eq!(
            parser_result,
            Err(nom::Err::Error(Error::from_error_kind("and events", Tag)))
        );
    }
}
