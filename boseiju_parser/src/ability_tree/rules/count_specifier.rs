use super::ParserNode;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "A" is the minimal count specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(
                &[ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id()],
            ),
            merged: ParserNode::CountSpecifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::A {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::CountSpecifier {
                    count: boseiju_tree::ability_tree::object::CountSpecifier::A {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "An" is also the minimal count specifier. Is this `allomorphy` ? */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::An {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::CountSpecifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::An {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::CountSpecifier {
                    count: boseiju_tree::ability_tree::object::CountSpecifier::A {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Numbers on their own can make count specifiers */
        /* Fixme: check what cards fails without it, maybe this is too much */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Number {
                number: Default::default(),
            }
            .id()]),
            merged: ParserNode::CountSpecifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Number { number }] => Ok(ParserNode::CountSpecifier {
                    count: boseiju_tree::ability_tree::object::CountSpecifier::Count(number.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* A count specifier can be made from a number and the special "target" word */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::Target {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::CountSpecifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::Target { .. })),
                ] => Ok(ParserNode::CountSpecifier {
                    count: boseiju_tree::ability_tree::object::CountSpecifier::Target(number.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "Target" alone is a shortcut for "a target" */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::CountSpecifier(
                intermediate::CountSpecifier::Target {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::CountSpecifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::Target {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::CountSpecifier {
                    count: boseiju_tree::ability_tree::object::CountSpecifier::Target(
                        boseiju_tree::ability_tree::number::Number::Number(boseiju_tree::ability_tree::number::FixedNumber {
                            number: 1,
                            #[cfg(feature = "spanned_tree")]
                            span: *span,
                        }),
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "All" is a count specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::All {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::CountSpecifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::CountSpecifier {
                    count: boseiju_tree::ability_tree::object::CountSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "The next" is a count specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::The {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Next {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::CountSpecifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::The {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Next {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::CountSpecifier {
                    count: boseiju_tree::ability_tree::object::CountSpecifier::TheNext {
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
