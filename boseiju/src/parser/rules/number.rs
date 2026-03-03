use super::ParserNode;
use crate::ability_tree::number;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let english_to_numbers_rules = vec![
        /* "An", "A" can be used as a number: "A card" really means "1 card" */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::An {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::An { span }))] => {
                    Ok(ParserNode::Number {
                        number: number::Number::Number(number::FixedNumber { number: 1, span: *span }),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { span }))] => {
                    Ok(ParserNode::Number {
                        number: number::Number::Number(number::FixedNumber { number: 1, span: *span }),
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "Each of up to" is an english formulation for the logical "up to" */
        /* Fixme: a bit of a shortcut, but is it fine ? */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                /* Fixme: each is parsed as an "all" ? */
                ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::All {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediates::Number::UpTo {
                    num: 0,
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::All { span: all_span })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of { .. })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::UpTo { num, span })),
                ] => Ok(ParserNode::Number {
                    number: crate::ability_tree::number::Number::UpTo(number::UpToNumber {
                        maximum: *num,
                        span: all_span.merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    let defined_numbers_rules = [
        intermediates::Number::Number {
            num: 0,
            span: Default::default(),
        },
        intermediates::Number::OrMore {
            num: 0,
            span: Default::default(),
        },
        intermediates::Number::UpTo {
            num: 0,
            span: Default::default(),
        },
        intermediates::Number::AnyNumber {
            span: Default::default(),
        },
        intermediates::Number::ThatMany {
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|number| super::ParserRule {
        expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::Number(number)).id()]),
        merged: ParserNode::Number { number: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(Token::Number(number))] => Ok(ParserNode::Number {
                number: match number {
                    intermediates::Number::Number { num, span } => {
                        crate::ability_tree::number::Number::Number(number::FixedNumber {
                            number: *num,
                            span: *span,
                        })
                    }
                    intermediates::Number::OrMore { num, span } => {
                        crate::ability_tree::number::Number::OrMore(number::OrMoreNumber {
                            minimum: *num,
                            span: *span,
                        })
                    }
                    intermediates::Number::UpTo { num, span } => crate::ability_tree::number::Number::UpTo(number::UpToNumber {
                        maximum: *num,
                        span: *span,
                    }),
                    intermediates::Number::AnyNumber { span } => crate::ability_tree::number::Number::AnyNumber { span: *span },
                    intermediates::Number::ThatMany { span } => crate::ability_tree::number::Number::ThatMany { span: *span },
                    _ => return Err("Unreachable in number rule"),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    [english_to_numbers_rules, defined_numbers_rules].into_iter().flatten()
}
