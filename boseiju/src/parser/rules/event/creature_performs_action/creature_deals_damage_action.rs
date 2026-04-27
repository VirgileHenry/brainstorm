use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    crate::ability_tree::terminals::DamageKind::all().flat_map(|damage_kind| {
        [
            /* "<creature reference> deals <damage kind>" */
            ParserRule {
                expanded: RuleLhs::new(&[
                    ParserNode::CreatureReference { creature: dummy() }.id(),
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Deals {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::DamageKind(damage_kind)).id(),
                ]),
                merged: ParserNode::Event { event: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::CreatureReference { creature },
                        ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Deals { .. })),
                        ParserNode::LexerToken(Token::DamageKind(damage_kind)),
                    ] => Ok(ParserNode::Event {
                        event: crate::ability_tree::event::Event::CreaturePerformsAction(
                            crate::ability_tree::event::CreaturePerformsActionEvent {
                                action: crate::ability_tree::action::CreatureAction::DealsDamage(
                                    crate::ability_tree::action::CreatureDealsDamageAction {
                                        creature: creature.clone(),
                                        damage_kind: damage_kind.clone(),
                                        to_player: None,
                                        #[cfg(feature = "spanned_tree")]
                                        span: creature.node_span().merge(&damage_kind.node_span()),
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.node_span().merge(&damage_kind.node_span()),
                            },
                        ),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
            /* "<creature reference> deals to <player>" */
            ParserRule {
                expanded: RuleLhs::new(&[
                    ParserNode::CreatureReference { creature: dummy() }.id(),
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Deals {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::DamageKind(damage_kind)).id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::Player { player: dummy() }.id(),
                ]),
                merged: ParserNode::Event { event: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::CreatureReference { creature },
                        ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Deals { .. })),
                        ParserNode::LexerToken(Token::DamageKind(damage_kind)),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To { .. })),
                        ParserNode::Player { player },
                    ] => Ok(ParserNode::Event {
                        event: crate::ability_tree::event::Event::CreaturePerformsAction(
                            crate::ability_tree::event::CreaturePerformsActionEvent {
                                action: crate::ability_tree::action::CreatureAction::DealsDamage(
                                    crate::ability_tree::action::CreatureDealsDamageAction {
                                        creature: creature.clone(),
                                        damage_kind: damage_kind.clone(),
                                        to_player: Some(player.clone()),
                                        #[cfg(feature = "spanned_tree")]
                                        span: creature.node_span().merge(&damage_kind.node_span()),
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.node_span().merge(&damage_kind.node_span()),
                            },
                        ),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
        ]
    })
}
