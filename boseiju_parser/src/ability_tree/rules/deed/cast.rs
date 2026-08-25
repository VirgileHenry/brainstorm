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
        /* "<active player> cast <passive spell>" is an active cast spell deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PlayerActive {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                    tense: boseiju_lexer::Tense::BaseForm,
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::SpellPassive {
                    spell: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::DeedCastActive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PlayerActive { player },
                    ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Cast,
                                ..
                            },
                    })),
                    ParserNode::SpellPassive { spell },
                ] => Ok(ParserNode::DeedCastActive {
                    deed: boseiju_tree::ability_tree::deed::cast::Cast {
                        caster: player.clone(),
                        spell: spell.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: player.span().merge(&spell.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<passive player> cast <passive spell>" is an passive cast spell deed */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PlayerPassive {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                    tense: boseiju_lexer::Tense::BaseForm,
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                }))
                .id(),
                ParserNode::SpellPassive {
                    spell: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::DeedCastPassive {
                deed: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PlayerPassive { player },
                    ParserNode::LexerToken(boseiju_lexer::Token::KeywordAction(intermediate::TensedKeywordAction {
                        tense: boseiju_lexer::Tense::BaseForm,
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Cast,
                                ..
                            },
                    })),
                    ParserNode::SpellPassive { spell },
                ] => Ok(ParserNode::DeedCastPassive {
                    deed: boseiju_tree::ability_tree::deed::cast::Cast {
                        caster: player.clone(),
                        spell: spell.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: player.span().merge(&spell.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
