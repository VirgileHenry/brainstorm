use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "destroy <active permanent>" is an active destroy deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                    tense: boseiju_lexer::Tense::BaseForm,
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Destroy,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::PermanentActive {
                    permanent: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::DeedDestroyActive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Destroy,
                                #[cfg(feature = "spanned_tree")]
                                    span: destroy_span,
                            },
                    })),
                    ParserNode::PermanentActive { permanent },
                ] => Ok(ParserNode::DeedDestroyActive {
                    deed: boseiju_tree::ability_tree::deed::destroy::Destroy {
                        object: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(destroy_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<active permanent> is destroyed" is a passive destroy deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PermanentPassive {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    token: intermediate::EnglishVerb::Be {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                    tense: boseiju_lexer::Tense::SimplePast,
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Destroy,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
            ]),
            merged: ParserNode::DeedDestroyPassive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PermanentPassive { permanent },
                    ParserNode::LexerToken(boseiju_lexer::Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                        token: intermediate::EnglishVerb::Be { .. },
                    })),
                    ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                        tense: boseiju_lexer::Tense::SimplePast,
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Destroy,
                                #[cfg(feature = "spanned_tree")]
                                    span: destroy_span,
                            },
                    })),
                ] => Ok(ParserNode::DeedDestroyPassive {
                    deed: boseiju_tree::ability_tree::deed::destroy::Destroy {
                        object: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(destroy_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive permanent> is destroyed" is a passive destroy deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PermanentPassive {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    token: intermediate::EnglishVerb::Be {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                    tense: boseiju_lexer::Tense::SimplePast,
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Destroy,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
            ]),
            merged: ParserNode::DeedDestroyPassive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PermanentPassive { permanent },
                    ParserNode::LexerToken(boseiju_lexer::Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                        token: intermediate::EnglishVerb::Be { .. },
                    })),
                    ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                        tense: boseiju_lexer::Tense::SimplePast,
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Destroy,
                                #[cfg(feature = "spanned_tree")]
                                    span: destroy_span,
                            },
                    })),
                ] => Ok(ParserNode::DeedDestroyPassive {
                    deed: boseiju_tree::ability_tree::deed::destroy::Destroy {
                        object: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(destroy_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive creature> dies" is a passive destroy deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreaturePassive {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::AmbiguousToken(intermediate::AmbiguousToken::Die {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::DeedDestroyPassive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreaturePassive { creature },
                    ParserNode::LexerToken(boseiju_lexer::Token::AmbiguousToken(intermediate::AmbiguousToken::Die {
                        #[cfg(feature = "spanned_tree")]
                            span: die_span,
                    })),
                ] => Ok(ParserNode::DeedDestroyPassive {
                    deed: boseiju_tree::ability_tree::deed::destroy::Destroy {
                        object: creature.to_permanent(),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(die_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
