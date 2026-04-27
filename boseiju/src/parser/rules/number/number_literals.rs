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
    [
        /* "<number>" number */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Number(intermediates::Number::Number {
                num: 0,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Number(intermediates::Number::Number {
                        num: fixed_number,
                        #[cfg(feature = "spanned_tree")]
                            span: number_span,
                    })),
                ] => Ok(ParserNode::Number {
                    number: number::Number::Number(number::FixedNumber {
                        number: *fixed_number,
                        #[cfg(feature = "spanned_tree")]
                        span: *number_span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<number> or more" number */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Number(intermediates::Number::OrMore {
                num: 0,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Number(intermediates::Number::OrMore {
                        num: fixed_number,
                        #[cfg(feature = "spanned_tree")]
                            span: number_span,
                    })),
                ] => Ok(ParserNode::Number {
                    number: number::Number::OrMore(number::OrMoreNumber {
                        minimum: *fixed_number,
                        #[cfg(feature = "spanned_tree")]
                        span: *number_span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "up to <number>" number */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Number(intermediates::Number::UpTo {
                num: 0,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Number(intermediates::Number::UpTo {
                        num: fixed_number,
                        #[cfg(feature = "spanned_tree")]
                            span: number_span,
                    })),
                ] => Ok(ParserNode::Number {
                    number: number::Number::UpTo(number::UpToNumber {
                        maximum: *fixed_number,
                        #[cfg(feature = "spanned_tree")]
                        span: *number_span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "up to <number>" number */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Number(intermediates::Number::AnyNumber {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Number(intermediates::Number::AnyNumber {
                        #[cfg(feature = "spanned_tree")]
                            span: number_span,
                    })),
                ] => Ok(ParserNode::Number {
                    number: number::Number::AnyNumber {
                        #[cfg(feature = "spanned_tree")]
                        span: *number_span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "each of up to" is an english formulation for the logical "up to" */
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
        /* "that many" number */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Number(intermediates::Number::ThatMany {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Number(intermediates::Number::ThatMany {
                        #[cfg(feature = "spanned_tree")]
                            span: number_span,
                    })),
                ] => Ok(ParserNode::Number {
                    number: number::Number::ThatMany {
                        #[cfg(feature = "spanned_tree")]
                        span: *number_span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
