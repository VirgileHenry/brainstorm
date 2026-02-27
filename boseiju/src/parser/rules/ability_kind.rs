use super::ParserNode;
use crate::{
    lexer::tokens::{TokenKind, non_terminals},
    utils::dummy,
};
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let ability_kind_rules = vec![
        /* Ability as an written ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Ability { ability: dummy() }.id()]),
            merged: ParserNode::AbilityKind { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Ability { ability }] => Ok(ParserNode::AbilityKind {
                    ability: crate::ability_tree::ability::AbilityKind::Written(ability.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Ability as a keyword ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::KeywordAbility { ability: dummy() }.id()]),
            merged: ParserNode::AbilityKind { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::KeywordAbility { ability }] => Ok(ParserNode::AbilityKind {
                    ability: crate::ability_tree::ability::AbilityKind::Keyword(ability.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    /* Ability Word with the ability, with a em dash */
    let ability_word_to_ability = mtg_data::AbilityWord::all()
        .map(|ab_word| super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::AbilityWord(ab_word)).id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::LongDash)).id(),
                ParserNode::Ability { ability: dummy() }.id(),
            ]),
            merged: ParserNode::AbilityKind { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::AbilityWord(ab_word)),
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::LongDash)),
                    ParserNode::Ability { ability },
                ] => Ok(ParserNode::AbilityKind {
                    ability: crate::ability_tree::ability::AbilityKind::AbilityWord(
                        crate::ability_tree::ability::AbilityWordAbility {
                            word: ab_word.clone(),
                            ability: ability.clone(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    [ability_kind_rules, ability_word_to_ability].into_iter().flatten()
}
