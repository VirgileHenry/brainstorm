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
        /* "<quantifier active> <specified artifact>" is an active artifact */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::QuantifierActive {
                    count: Default::default(),
                }
                .id(),
                ParserNode::SpecifiedArtifact {
                    artifact: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ArtifactActive {
                artifact: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::QuantifierActive { count },
                    ParserNode::SpecifiedArtifact { artifact },
                ] => Ok(ParserNode::ArtifactActive {
                    artifact: object::Artifact::Reference(object::reference::ArtifactReference {
                        quantifier: count.clone(),
                        artifact: artifact.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(&artifact.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<quantifier passive> <specified artifact>" is a passive artifact */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::QuantifierPassive {
                    count: Default::default(),
                }
                .id(),
                ParserNode::SpecifiedArtifact {
                    artifact: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ArtifactPassive {
                artifact: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::QuantifierPassive { count },
                    ParserNode::SpecifiedArtifact { artifact },
                ] => Ok(ParserNode::ArtifactPassive {
                    artifact: object::Artifact::Reference(object::reference::ArtifactReference {
                        quantifier: count.clone(),
                        artifact: artifact.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(&artifact.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "another <specified artifact>" is a + other artifact */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedArtifact {
                    artifact: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ArtifactPassive {
                artifact: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                        #[cfg(feature = "spanned_tree")]
                            span: another_span,
                    })),
                    ParserNode::SpecifiedArtifact { artifact },
                ] => Ok(ParserNode::ArtifactPassive {
                    artifact: object::Artifact::Reference(object::reference::ArtifactReference {
                        quantifier: quantifier::PassiveQuantifier::Count(quantifier::CountQuantifier {
                            number: boseiju_tree::ability_tree::number::Number::Flat(
                                boseiju_tree::ability_tree::number::FlatNumber {
                                    number: 1,
                                    #[cfg(feature = "spanned_tree")]
                                    span: *another_span,
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: *another_span,
                        }),
                        artifact: artifact.add_factor_specifier(object::specified_object::ArtifactSpecifier::Another(
                            object::specified_object::AnotherObjectSpecifier {
                                #[cfg(feature = "spanned_tree")]
                                span: *another_span,
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: artifact.span().merge(another_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified artifact>" is a artifact with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedArtifact {
                artifact: Default::default(),
            }
            .id()]),
            merged: ParserNode::ArtifactActive {
                artifact: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedArtifact { artifact }] => Ok(ParserNode::ArtifactActive {
                    artifact: object::Artifact::Reference(object::reference::ArtifactReference {
                        quantifier: quantifier::ActiveQuantifier::All(quantifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: artifact.span().empty_at_start(),
                        }),
                        artifact: artifact.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: artifact.span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <specified artifact>" can be used as a artifact reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDemonstrative(intermediate::EnglishDemonstrative::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedArtifact {
                    artifact: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ArtifactPassive {
                artifact: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDemonstrative(intermediate::EnglishDemonstrative::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::SpecifiedArtifact {
                        #[cfg(feature = "spanned_tree")]
                        artifact,
                        ..
                    },
                ] => Ok(ParserNode::ArtifactPassive {
                    artifact: object::Artifact::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: artifact.span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "it" makes a previously mentionned artifact */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ArtifactActive {
                artifact: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishPronoun(intermediate::EnglishPronoun::It {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::ArtifactActive {
                    artifact: object::Artifact::PreviouslyMentionned(object::PreviouslyMentionned {
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
