use crate::ability_tree::number;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let english_to_numbers_rules = vec![
        /* "An", "A" can be used as a number: "A card" really means "1 card" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::An {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::An {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Number {
                    number: number::Number::Number(number::FixedNumber {
                        number: 1,
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Number {
                    number: number::Number::Number(number::FixedNumber {
                        number: 1,
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* X on its own means that the x had to be in the cost */
        /* Fixme: maybe context could help to ensure that's the case ? */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: x_span,
                    })),
                ] => Ok(ParserNode::Number {
                    number: number::Number::X(number::XNumber {
                        x_definition: Box::new(crate::ability_tree::number::XDefinition::FromCost {
                            #[cfg(feature = "spanned_tree")]
                            span: x_span.clone(),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: x_span.clone(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "Each of up to" is an english formulation for the logical "up to" */
        /* Fixme: a bit of a shortcut, but is it fine ? */
        ParserRule {
            expanded: RuleLhs::new(&[
                /* Fixme: each is parsed as an "all" ? */
                ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::All {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediates::Number::UpTo {
                    num: 0,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                            span: all_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Of { .. })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::UpTo {
                        num,
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Number {
                    number: crate::ability_tree::number::Number::UpTo(number::UpToNumber {
                        maximum: *num,
                        #[cfg(feature = "spanned_tree")]
                        span: all_span.merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    let defined_numbers_rules = [
        intermediates::Number::Number {
            num: 0,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        intermediates::Number::OrMore {
            num: 0,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        intermediates::Number::UpTo {
            num: 0,
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        intermediates::Number::AnyNumber {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        intermediates::Number::ThatMany {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|number| ParserRule {
        expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Number(number)).id()]),
        merged: ParserNode::Number { number: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(Token::Number(number))] => Ok(ParserNode::Number {
                number: match number {
                    intermediates::Number::Number {
                        num,
                        #[cfg(feature = "spanned_tree")]
                        span,
                    } => crate::ability_tree::number::Number::Number(number::FixedNumber {
                        number: *num,
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                    intermediates::Number::OrMore {
                        num,
                        #[cfg(feature = "spanned_tree")]
                        span,
                    } => crate::ability_tree::number::Number::OrMore(number::OrMoreNumber {
                        minimum: *num,
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                    intermediates::Number::UpTo {
                        num,
                        #[cfg(feature = "spanned_tree")]
                        span,
                    } => crate::ability_tree::number::Number::UpTo(number::UpToNumber {
                        maximum: *num,
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                    intermediates::Number::AnyNumber {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    } => crate::ability_tree::number::Number::AnyNumber {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                    intermediates::Number::ThatMany {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    } => crate::ability_tree::number::Number::ThatMany {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                    _ => return Err("Unreachable in number rule"),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    [english_to_numbers_rules, defined_numbers_rules].into_iter().flatten()
}
