use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Creature does a creature event */
        /* Creature deals combat damage */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Deals {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::DamageKind(intermediates::DamageKind::CombatDamage {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Deals { span: sp1 })),
                    ParserNode::LexerToken(Token::DamageKind(intermediates::DamageKind::CombatDamage { span: sp2 })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreatureAction(crate::ability_tree::event::CreatureActionEvent {
                        creatures: reference.clone(),
                        action: crate::ability_tree::event::CreatureAction::DealsCombatDamage(
                            crate::ability_tree::event::CreatureDealsCombatDamageAction { span: sp1.merge(sp2) },
                        ),
                        span: reference.span().merge(sp2),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Creature attacks action */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Attack {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Attack { span })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreatureAction(crate::ability_tree::event::CreatureActionEvent {
                        creatures: reference.clone(),
                        action: crate::ability_tree::event::CreatureAction::Attacks(
                            crate::ability_tree::event::CreatureAttacksAction {
                                attacked_player: None,
                                span: *span,
                            },
                        ),
                        span: reference.span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Creature dies action */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Dies {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Dies { span })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreatureAction(crate::ability_tree::event::CreatureActionEvent {
                        creatures: reference.clone(),
                        action: crate::ability_tree::event::CreatureAction::Dies(
                            crate::ability_tree::event::CreatureDiesAction { span: *span },
                        ),
                        span: reference.span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
