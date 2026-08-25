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
        /* "<active player ref> sacrifices <passive permanent>" is an active sacrifice deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PlayerActive {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Sacrifice,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::PermanentPassive {
                    permanent: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::DeedSacrificeActive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PlayerActive { player },
                    ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Sacrifice,
                                ..
                            },
                    })),
                    ParserNode::PermanentPassive { permanent },
                ] => Ok(ParserNode::DeedSacrificeActive {
                    deed: boseiju_tree::ability_tree::deed::sacrifice::Sacrifice {
                        player: player.clone(),
                        permanent: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: player.span().merge(&permanent.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive player ref> sacrifices <passive permanent>" is a passive sacrifice deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PlayerPassive {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Sacrifice,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::PermanentPassive {
                    permanent: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::DeedSacrificePassive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PlayerPassive { player },
                    ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Sacrifice,
                                ..
                            },
                    })),
                    ParserNode::PermanentPassive { permanent },
                ] => Ok(ParserNode::DeedSacrificePassive {
                    deed: boseiju_tree::ability_tree::deed::sacrifice::Sacrifice {
                        player: player.clone(),
                        permanent: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: player.span().merge(&permanent.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "sacrifice <passive permanent>" is a sac deed with an implicit "you" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                    tense: boseiju_lexer::Tense::BaseForm,
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Sacrifice,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::PermanentPassive {
                    permanent: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::DeedSacrificePassive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Sacrifice,
                                #[cfg(feature = "spanned_tree")]
                                    span: sac_span,
                            },
                    })),
                    ParserNode::PermanentPassive { permanent },
                ] => Ok(ParserNode::DeedSacrificePassive {
                    deed: boseiju_tree::ability_tree::deed::sacrifice::Sacrifice {
                        player: boseiju_tree::ability_tree::player::PlayerReference::You(
                            boseiju_tree::ability_tree::player::You {
                                #[cfg(feature = "spanned_tree")]
                                span: sac_span.empty_at_start(),
                            },
                        ),
                        permanent: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(sac_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
