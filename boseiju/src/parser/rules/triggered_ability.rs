use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Whenever, an event, a comma and a statement make the structure for triggered abilities. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Whenever)).id(),
                ParserNode::TriggerCondition { condition: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Whenever)),
                    ParserNode::TriggerCondition { condition },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Triggered(
                        crate::ability_tree::ability::triggered::TriggeredAbility {
                            condition: condition.clone(),
                            effect: ability.clone(),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* When, an event, a comma and a statement also make a structure for triggered abilities. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::When)).id(),
                ParserNode::TriggerCondition { condition: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::When)),
                    ParserNode::TriggerCondition { condition },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Triggered(
                        crate::ability_tree::ability::triggered::TriggeredAbility {
                            condition: condition.clone(),
                            effect: ability.clone(),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Event on their own can make trigger conditions */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Event { event: dummy() }.id()]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Event { event }] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        event: event.clone(),
                        condition: None,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Event with a comma and a if condition can also make trigger conditions */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Event { event: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)).id(),
                ParserNode::Condition { condition: dummy() }.id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Event { event },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        event: event.clone(),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                            },
                        )),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Special case for the "during your turn" if condition that may not appear behind a comma */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Event { event: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::During)).id(),
                /* Fixme: a bit weird for a "your turn" ? Maybe it shall be a single token */
                ParserNode::LexerToken(TokenKind::OwnerSpecifier(terminals::OwnerSpecifier::YouOwn)).id(),
                ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Turn)).id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Event { event },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::During)),
                    /* Fixme: a bit weird for a "your turn" ? Maybe it shall be a single token */
                    ParserNode::LexerToken(TokenKind::OwnerSpecifier(terminals::OwnerSpecifier::YouOwn)),
                    ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Turn)),
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        event: event.clone(),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: crate::ability_tree::conditional::Condition::ThisIsYourTurn(
                                    crate::ability_tree::conditional::ConditionThisIsYourTurn,
                                ),
                            },
                        )),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
