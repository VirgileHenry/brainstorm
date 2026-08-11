use super::ParserNode;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "A" is the minimal count specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(
                &[ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id()],
            ),
            merged: ParserNode::Quantifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::A {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Quantifier {
                    count: boseiju_tree::ability_tree::quantifier::ActiveQuantifier::Count(
                        boseiju_tree::ability_tree::quantifier::CountQuantifier {
                            number: boseiju_tree::ability_tree::number::Number::Flat(
                                boseiju_tree::ability_tree::number::FixedNumber {
                                    number: 1,
                                    #[cfg(feature = "spanned_tree")]
                                    span: *span,
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: *span,
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "An" is also the minimal count specifier. Is this `allomorphy` ? */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::An {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Quantifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishArticle(intermediate::EnglishArticle::An {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Quantifier {
                    count: boseiju_tree::ability_tree::quantifier::ActiveQuantifier::Count(
                        boseiju_tree::ability_tree::quantifier::CountQuantifier {
                            number: boseiju_tree::ability_tree::number::Number::Flat(
                                boseiju_tree::ability_tree::number::FixedNumber {
                                    number: 1,
                                    #[cfg(feature = "spanned_tree")]
                                    span: *span,
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: *span,
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Numbers on their own can make count specifiers */
        /* Fixme: check what cards fails without it, maybe this is too much */
        /* Fixme: as sais in the quantifier node, maybe this is nooot super clean */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Number {
                number: Default::default(),
            }
            .id()]),
            merged: ParserNode::Quantifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Number { number }] => Ok(ParserNode::Quantifier {
                    count: boseiju_tree::ability_tree::quantifier::ActiveQuantifier::Count(
                        boseiju_tree::ability_tree::quantifier::CountQuantifier {
                            number: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.span(),
                        },
                    ),
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
            merged: ParserNode::Quantifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::Target { .. })),
                ] => Ok(ParserNode::Quantifier {
                    count: boseiju_tree::ability_tree::quantifier::ActiveQuantifier::Target(
                        boseiju_tree::ability_tree::quantifier::TargetQuantifier {
                            number: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.span(),
                        },
                    ),
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
            merged: ParserNode::Quantifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::Target {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Quantifier {
                    count: boseiju_tree::ability_tree::quantifier::ActiveQuantifier::Target(
                        boseiju_tree::ability_tree::quantifier::TargetQuantifier {
                            number: boseiju_tree::ability_tree::number::Number::Flat(
                                boseiju_tree::ability_tree::number::FixedNumber {
                                    number: 1,
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.empty_at_start(),
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: *span,
                        },
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
            merged: ParserNode::Quantifier {
                count: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CountSpecifier(intermediate::CountSpecifier::All {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Quantifier {
                    count: boseiju_tree::ability_tree::quantifier::ActiveQuantifier::All(
                        boseiju_tree::ability_tree::quantifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: *span,
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
