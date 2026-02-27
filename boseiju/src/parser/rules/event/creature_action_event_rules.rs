use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
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
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Deals)).id(),
                ParserNode::LexerToken(TokenKind::DamageKind(non_terminals::DamageKind::CombatDamage)).id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Deals)),
                    ParserNode::LexerToken(TokenKind::DamageKind(non_terminals::DamageKind::CombatDamage)),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreatureAction(crate::ability_tree::event::CreatureActionEvent {
                        creatures: reference.clone(),
                        action: crate::ability_tree::event::CreatureAction::DealsCombatDamage(
                            crate::ability_tree::event::CreatureDealsCombatDamageAction,
                        ),
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
                ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Attack)).id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Attack)),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreatureAction(crate::ability_tree::event::CreatureActionEvent {
                        creatures: reference.clone(),
                        action: crate::ability_tree::event::CreatureAction::Attacks(
                            crate::ability_tree::event::CreatureAttacksAction { attacked_player: None },
                        ),
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
                ParserNode::LexerToken(TokenKind::CardActions(non_terminals::CardActions::Dies)).id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::CardActions(non_terminals::CardActions::Dies)),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreatureAction(crate::ability_tree::event::CreatureActionEvent {
                        creatures: reference.clone(),
                        action: crate::ability_tree::event::CreatureAction::Dies(crate::ability_tree::event::CreatureDiesAction),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
