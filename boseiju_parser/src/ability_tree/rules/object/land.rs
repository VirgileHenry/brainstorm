use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::object;
use boseiju_tree::ability_tree::quantifier;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<count> <specified land>" is a land */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Quantifier {
                    count: Default::default(),
                }
                .id(),
                ParserNode::SpecifiedLand {
                    land: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Land {
                land: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Quantifier { count }, ParserNode::SpecifiedLand { land }] => Ok(ParserNode::Land {
                    land: object::Land::Reference(object::reference::LandReference {
                        count: count.clone(),
                        land: land.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(&land.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified land>" is a land with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedLand {
                land: Default::default(),
            }
            .id()]),
            merged: ParserNode::Land {
                land: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedLand { land }] => Ok(ParserNode::Land {
                    land: object::Land::Reference(object::reference::LandReference {
                        count: quantifier::Quantifier::All(quantifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: land.span().empty_at_start(),
                        }),
                        land: land.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: land.span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "another <specified land>" is a + other land */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedLand {
                    land: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Land {
                land: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                        #[cfg(feature = "spanned_tree")]
                            span: another_span,
                    })),
                    ParserNode::SpecifiedLand { land },
                ] => Ok(ParserNode::Land {
                    land: object::Land::Reference(object::reference::LandReference {
                        count: quantifier::Quantifier::Count(quantifier::CountQuantifier {
                            number: boseiju_tree::ability_tree::number::Number::Number(
                                boseiju_tree::ability_tree::number::FixedNumber {
                                    number: 1,
                                    #[cfg(feature = "spanned_tree")]
                                    span: *another_span,
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: *another_span,
                        }),
                        land: land.add_factor_specifier(object::specified_object::LandSpecifier::Another(
                            object::specified_object::AnotherObjectSpecifier {
                                #[cfg(feature = "spanned_tree")]
                                span: *another_span,
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: land.span().merge(another_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <specified land>" is a self referencing land */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDemonstrative(intermediate::EnglishDemonstrative::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedLand {
                    land: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Land {
                land: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDemonstrative(intermediate::EnglishDemonstrative::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::SpecifiedLand {
                        #[cfg(feature = "spanned_tree")]
                        land,
                        ..
                    },
                ] => Ok(ParserNode::Land {
                    land: object::Land::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: land.span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "fortified land" is an attached object land reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::AttachedObject(
                intermediate::AttachedObject::FortifiedLand {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Land {
                land: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AttachedObject(intermediate::AttachedObject::FortifiedLand {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Land {
                    land: object::Land::Attached(object::AttachedObject {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "it" makes a previously mentionned land */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Land {
                land: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::It {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Land {
                    land: object::Land::PreviouslyMentionned(object::PreviouslyMentionned {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
