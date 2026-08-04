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
        /* "<count> <specified permanent>" is a permanent */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Quantifier {
                    count: Default::default(),
                }
                .id(),
                ParserNode::SpecifiedPermanent {
                    permanent: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Quantifier { count }, ParserNode::SpecifiedPermanent { permanent }] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::Reference(object::reference::PermanentReference {
                        count: count.clone(),
                        permanent: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(&permanent.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified permanent>" is a permanent with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedPermanent {
                permanent: Default::default(),
            }
            .id()]),
            merged: ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedPermanent { permanent }] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::Reference(object::reference::PermanentReference {
                        count: quantifier::Quantifier::All(quantifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.span().empty_at_start(),
                        }),
                        permanent: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "another <specified permanent>" is a + other permanent */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedPermanent {
                    permanent: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                        #[cfg(feature = "spanned_tree")]
                            span: another_span,
                    })),
                    ParserNode::SpecifiedPermanent { permanent },
                ] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::Reference(object::reference::PermanentReference {
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
                        permanent: permanent.add_factor_specifier(object::specified_object::PermanentSpecifier::Another(
                            object::specified_object::AnotherObjectSpecifier {
                                #[cfg(feature = "spanned_tree")]
                                span: *another_span,
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(another_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <specified permanent>" is a self referencing permanent */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDemonstrative(intermediate::EnglishDemonstrative::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedPermanent {
                    permanent: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDemonstrative(intermediate::EnglishDemonstrative::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::SpecifiedPermanent {
                        #[cfg(feature = "spanned_tree")]
                        permanent,
                        ..
                    },
                ] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<card own name>" is a self referencing permanent */
        /* Fixme: this rule shall only be used when parsing permanents, otherwise cards may think they are permanents */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::CardOwnName(intermediate::CardOwnName {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::CardOwnName(intermediate::CardOwnName {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                ] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: *start_span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "enchanted <specified permanent>" is an attached permanent creature reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::AttachedObject(
                intermediate::AttachedObject::AttachedPermanent {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AttachedObject(intermediate::AttachedObject::AttachedPermanent {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::Attached(object::AttachedObject {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "enchanted creature" and "equipped creature" can be upscaled to enchanted permanents */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::AttachedObject(
                intermediate::AttachedObject::AttachedCreature {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AttachedObject(intermediate::AttachedObject::AttachedCreature {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::Attached(object::AttachedObject {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "fortified land" can be upscaled to attached permanent */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::AttachedObject(
                intermediate::AttachedObject::FortifiedLand {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AttachedObject(intermediate::AttachedObject::FortifiedLand {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::Attached(object::AttachedObject {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "it" makes a previously mentionned permanent */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Permanent {
                permanent: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::It {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Permanent {
                    permanent: object::Permanent::PreviouslyMentionned(object::PreviouslyMentionned {
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
