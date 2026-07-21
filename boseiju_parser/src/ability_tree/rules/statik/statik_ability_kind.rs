use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::ability::statik::alterative_casting_permissions;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* Continuous effect make a static ability kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ContinuousEffect {
                effect: Default::default(),
            }
            .id()]),
            merged: ParserNode::StaticAbilityKind {
                kind: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ContinuousEffect { effect }] => Ok(ParserNode::StaticAbilityKind {
                    kind: boseiju_tree::ability_tree::ability::statik::StaticAbilityKind::ContinuousEffect(effect.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Cost modifications effects make a static aility kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::CostModificationEffect {
                cost_modification: Default::default(),
            }
            .id()]),
            merged: ParserNode::StaticAbilityKind {
                kind: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CostModificationEffect { cost_modification }] => Ok(ParserNode::StaticAbilityKind {
                    kind: boseiju_tree::ability_tree::ability::statik::StaticAbilityKind::CostModificationEffect(
                        cost_modification.clone(),
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Alternative casting permissions make static ability kind */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::Card {
                    card: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::From {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference {
                    zone: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::StaticAbilityKind {
                kind: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May { .. })),
                    ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Cast,
                                ..
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::Card { card },
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::From { .. })),
                    ParserNode::ZoneReference { zone },
                ] => Ok(ParserNode::StaticAbilityKind {
                    kind: boseiju_tree::ability_tree::ability::statik::StaticAbilityKind::AlternativeCastingPermissions(
                        alterative_casting_permissions::AlternativeCastingPermissions {
                            player: player.clone(),
                            object: card.clone(),
                            from_zone: zone.clone(),
                            additional_cost: None,
                            #[cfg(feature = "spanned_tree")]
                            span: player.span().merge(&zone.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Alternative casting permissions with additionnal cost make static ability kind */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                    token: intermediate::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Cast,
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::Card {
                    card: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::From {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference {
                    zone: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::By {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Cost {
                    cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::InAdditionTo {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                    token: intermediate::PlayerAction::Pay {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::PresentParticiple,
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Its {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Other {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardProperty(intermediate::CardProperty::Cost {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::StaticAbilityKind {
                kind: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::May { .. })),
                    ParserNode::LexerToken(Token::TensedKeywordAction(intermediate::TensedKeywordAction {
                        token:
                            intermediate::KeywordAction {
                                keyword_action: mtg_data::KeywordAction::Cast,
                                ..
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::Card { card },
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::From { .. })),
                    ParserNode::ZoneReference { zone },
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::By { .. })),
                    ParserNode::Cost { cost: inner },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::InAdditionTo { .. })),
                    ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                        token: intermediate::PlayerAction::Pay { .. },
                        tense: boseiju_lexer::Tense::PresentParticiple,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Its { .. })),
                    ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Other { .. })),
                    ParserNode::LexerToken(Token::CardProperty(intermediate::CardProperty::Cost {
                        #[cfg(feature = "spanned_tree")]
                            span: in_addition_span,
                    })),
                ] => Ok(ParserNode::StaticAbilityKind {
                    kind: boseiju_tree::ability_tree::ability::statik::StaticAbilityKind::AlternativeCastingPermissions(
                        alterative_casting_permissions::AlternativeCastingPermissions {
                            player: player.clone(),
                            object: card.clone(),
                            from_zone: zone.clone(),
                            additional_cost: Some(inner.clone()),
                            #[cfg(feature = "spanned_tree")]
                            span: player.span().merge(in_addition_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
