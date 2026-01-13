use super::ParserNode;
use super::uninit;

use idris::Idris;

use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;

pub fn ability_tree_rules() -> impl Iterator<Item = super::ParserRule> {
    [
        /* Statements are spell abilities */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::Statement { statement: uninit() }.id()]),
            result: ParserNode::Ability { ability: uninit() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Statement { statement }] => Some(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Spell(
                        crate::ability_tree::ability::spell::SpellAbility {
                            effect: statement.clone(),
                        },
                    )),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Triggered abilities from all their keywords: When, Whenever */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::When)).id(),
                ParserNode::TriggerCondition { condition: uninit() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                ParserNode::Statement { statement: uninit() }.id(),
            ]),
            result: ParserNode::Ability { ability: uninit() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Whenever)),
                    ParserNode::TriggerCondition { condition },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                    ParserNode::Statement { statement },
                ] => Some(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Triggered(
                        crate::ability_tree::ability::triggered::TriggeredAbility {
                            condition: condition.clone(),
                            effect: statement.clone(),
                        },
                    )),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Continuous effects are static abilities */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::ContinuousEffect { effect: uninit() }.id()]),
            result: ParserNode::Ability { ability: uninit() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ContinuousEffect { effect }] => Some(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Static(
                        crate::ability_tree::ability::statik::StaticAbility::ContinuousEffect(effect.clone()),
                    )),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Characteristic defining ability is an ability */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::CharacteristicDefiningAbility { ability: uninit() }.id()]),
            result: ParserNode::Ability { ability: uninit() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CharacteristicDefiningAbility { ability }] => Some(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Static(
                        crate::ability_tree::ability::statik::StaticAbility::CharasteristicDefiningAbility(ability.clone()),
                    )),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* A single Ability can be turned into an ability tree with a single element */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::Ability { ability: uninit() }.id()]),
            result: ParserNode::AbilityTree { tree: uninit() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Ability { ability }] => Some(ParserNode::AbilityTree {
                    tree: {
                        let mut abilities = arrayvec::ArrayVec::new();
                        abilities.push(*ability.clone());
                        Box::new(crate::AbilityTree { abilities })
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Abilities separated by new lines can be merged into a single ability tree */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::AbilityTree { tree: uninit() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)).id(),
                ParserNode::Ability { ability: uninit() }.id(),
            ]),
            result: ParserNode::AbilityTree { tree: uninit() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::AbilityTree { tree },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::NewLine)),
                    ParserNode::Ability { ability },
                ] => Some(ParserNode::AbilityTree {
                    tree: {
                        let mut abilities = tree.abilities.clone();
                        abilities.push(*ability.clone());
                        Box::new(crate::AbilityTree { abilities })
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
