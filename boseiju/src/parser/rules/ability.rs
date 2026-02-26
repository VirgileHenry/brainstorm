use super::ParserNode;
use crate::{
    lexer::tokens::{TokenKind, non_terminals},
    utils::dummy,
};
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Spell ability to ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::SpellAbility { ability: dummy() }.id()]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpellAbility { ability }] => Ok(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Spell(ability.clone())),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Static ability kind to static ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::StaticAbilityKind { kind: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::StaticAbilityKind { kind },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
                ] => Ok(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Static(
                        crate::ability_tree::ability::statik::StaticAbility {
                            kind: kind.clone(),
                            condition: None,
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Static ability kind with a condition to static ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::StaticAbilityKind { kind: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)).id(),
                ParserNode::Condition { condition: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::StaticAbilityKind { kind },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)),
                    ParserNode::Condition { condition },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
                ] => Ok(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Static(
                        crate::ability_tree::ability::statik::StaticAbility {
                            kind: kind.clone(),
                            condition: Some(crate::ability_tree::conditional::Conditional::If(
                                crate::ability_tree::conditional::ConditionalIf {
                                    condition: condition.clone(),
                                },
                            )),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Static ability kind with a condition before and a comma to static ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)).id(),
                ParserNode::Condition { condition: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                ParserNode::StaticAbilityKind { kind: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)),
                    ParserNode::Condition { condition },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                    ParserNode::StaticAbilityKind { kind },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
                ] => Ok(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Static(
                        crate::ability_tree::ability::statik::StaticAbility {
                            kind: kind.clone(),
                            condition: Some(crate::ability_tree::conditional::Conditional::If(
                                crate::ability_tree::conditional::ConditionalIf {
                                    condition: condition.clone(),
                                },
                            )),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
