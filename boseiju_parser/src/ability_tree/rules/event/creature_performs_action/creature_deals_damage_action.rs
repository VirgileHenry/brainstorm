use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::action;
use boseiju_tree::ability_tree::event;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    boseiju_lexer::terminal::DamageKind::all().flat_map(|damage_kind| {
        [
            /* "<creature reference> deals <damage kind>" */
            ParserRule {
                expanded: RuleLhs::new(&[
                    ParserNode::CreaturePassive {
                        creature: Default::default(),
                    }
                    .id(),
                    ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                        token: intermediate::ActionKeyword::Deals {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::DamageKind(damage_kind)).id(),
                ]),
                merged: ParserNode::Event {
                    event: Default::default(),
                }
                .id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::CreaturePassive { creature },
                        ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                            token: intermediate::ActionKeyword::Deals { .. },
                            tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                        })),
                        ParserNode::LexerToken(Token::DamageKind(damage_kind)),
                    ] => Ok(ParserNode::Event {
                        event: event::Event::CreaturePerformsAction(event::CreaturePerformsActionEvent {
                            action: action::CreatureAction::DealsDamage(action::CreatureDealsDamageAction {
                                creature: creature.clone(),
                                damage_kind: damage_kind.clone(),
                                to_player: None,
                                #[cfg(feature = "spanned_tree")]
                                span: creature.span().merge(&damage_kind.span()),
                            }),
                            #[cfg(feature = "spanned_tree")]
                            span: creature.span().merge(&damage_kind.span()),
                        }),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
            /* "<creature reference> deals to <player>" */
            ParserRule {
                expanded: RuleLhs::new(&[
                    ParserNode::CreaturePassive {
                        creature: Default::default(),
                    }
                    .id(),
                    ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                        token: intermediate::ActionKeyword::Deals {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::DamageKind(damage_kind)).id(),
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::PlayerPassive {
                        player: Default::default(),
                    }
                    .id(),
                ]),
                merged: ParserNode::Event {
                    event: Default::default(),
                }
                .id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::CreaturePassive { creature },
                        ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                            token: intermediate::ActionKeyword::Deals { .. },
                            tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                        })),
                        ParserNode::LexerToken(Token::DamageKind(damage_kind)),
                        ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To { .. })),
                        ParserNode::PlayerPassive { player },
                    ] => Ok(ParserNode::Event {
                        event: event::Event::CreaturePerformsAction(event::CreaturePerformsActionEvent {
                            action: action::CreatureAction::DealsDamage(action::CreatureDealsDamageAction {
                                creature: creature.clone(),
                                damage_kind: damage_kind.clone(),
                                to_player: Some(player.clone()),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.span().merge(&damage_kind.span()),
                            }),
                            #[cfg(feature = "spanned_tree")]
                            span: creature.span().merge(&damage_kind.span()),
                        }),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
        ]
    })
}
