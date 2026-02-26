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
        /* We need to keep the creature specifier visible for this */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                /* Specifically "creature" here */
                ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                    mtg_data::CardType::Creature,
                )))
                .id(),
                ParserNode::CreatureAction { action: dummy() }.id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CountSpecifier { count },
                    /* Specifically "creature" here */
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                        mtg_data::CardType::Creature,
                    ))),
                    ParserNode::CreatureAction { action },
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreatureAction(crate::ability_tree::event::CreatureActionEvent {
                        creatures: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                            crate::ability_tree::object::SpecifiedObject {
                                amount: count.clone(),
                                specifiers: crate::ability_tree::object::ObjectSpecifiers::creature(),
                            },
                        ),
                        action: action.clone(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Same with object specifiers before the "creature" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
                /* Specifically "creature" here */
                ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                    mtg_data::CardType::Creature,
                )))
                .id(),
                ParserNode::CreatureAction { action: dummy() }.id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CountSpecifier { count },
                    ParserNode::ObjectSpecifiers { specifiers },
                    /* Specifically "creature" here */
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                        mtg_data::CardType::Creature,
                    ))),
                    ParserNode::CreatureAction { action },
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreatureAction(crate::ability_tree::event::CreatureActionEvent {
                        creatures: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                            crate::ability_tree::object::SpecifiedObject {
                                amount: count.clone(),
                                specifiers: specifiers
                                    .add_factor_specifier(crate::ability_tree::object::ObjectSpecifier::creature()),
                            },
                        ),
                        action: action.clone(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Same with object specifiers after the "creature" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                /* Specifically "creature" here */
                ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                    mtg_data::CardType::Creature,
                )))
                .id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
                ParserNode::CreatureAction { action: dummy() }.id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CountSpecifier { count },
                    /* Specifically "creature" here */
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                        mtg_data::CardType::Creature,
                    ))),
                    ParserNode::ObjectSpecifiers { specifiers },
                    ParserNode::CreatureAction { action },
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreatureAction(crate::ability_tree::event::CreatureActionEvent {
                        creatures: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                            crate::ability_tree::object::SpecifiedObject {
                                amount: count.clone(),
                                specifiers: specifiers
                                    .add_factor_specifier(crate::ability_tree::object::ObjectSpecifier::creature()),
                            },
                        ),
                        action: action.clone(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Fixme: maybe at some point we will also need specifiers before and afters ? */

        /* Creature deals combat damage action */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Deals)).id(),
                ParserNode::LexerToken(TokenKind::DamageKind(non_terminals::DamageKind::CombatDamage)).id(),
            ]),
            merged: ParserNode::CreatureAction { action: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Deals)),
                    ParserNode::LexerToken(TokenKind::DamageKind(non_terminals::DamageKind::CombatDamage)),
                ] => Ok(ParserNode::CreatureAction {
                    action: crate::ability_tree::event::CreatureAction::DealsCombatDamage(
                        crate::ability_tree::event::CreatureDealsCombatDamageAction,
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
