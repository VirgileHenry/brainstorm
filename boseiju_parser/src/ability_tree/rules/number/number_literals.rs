use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::number;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<number>" number */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Number(intermediate::Number::NumberLiteral {
                num: 0,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Number {
                number: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Number(intermediate::Number::NumberLiteral {
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
        /*
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Number(intermediate::Number::OrMore {
                num: 0,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Number { number: Default::default() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Number(intermediate::Number::OrMore {
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
         */
        /* "any number" number */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Number(intermediate::Number::AnyNumber {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Number {
                number: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Number(intermediate::Number::AnyNumber {
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
        /*
        ParserRule {
            expanded: RuleLhs::new(&[
                /* Fixme: each is parsed as an "all" ? */
                ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::All {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediate::Number::UpTo {
                    num: 0,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Number { number: Default::default() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                            span: all_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::Of { .. })),
                    ParserNode::LexerToken(Token::Number(intermediate::Number::UpTo {
                        num,
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Number {
                    number: boseiju_tree::ability_tree::number::Number::UpTo(number::UpToNumber {
                        maximum: *num,
                        #[cfg(feature = "spanned_tree")]
                        span: all_span.merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        */
    ]
    .into_iter()
}
