use super::ParserNode;
use crate::{
    lexer::tokens::{TokenKind, non_terminals},
    utils::dummy,
};
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let static_ability_kind_rules = vec![
        /* Continuous effect make a static ability kind */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::ContinuousEffect { effect: dummy() }.id()]),
            merged: ParserNode::StaticAbilityKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ContinuousEffect { effect }] => Ok(ParserNode::StaticAbilityKind {
                    kind: crate::ability_tree::ability::statik::StaticAbilityKind::ContinuousEffect(effect.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Cost modifications effects make a static aility kind */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::CostModificationEffect {
                cost_modification: dummy(),
            }
            .id()]),
            merged: ParserNode::StaticAbilityKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CostModificationEffect { cost_modification }] => Ok(ParserNode::StaticAbilityKind {
                    kind: crate::ability_tree::ability::statik::StaticAbilityKind::CostModificationEffect(
                        cost_modification.clone(),
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    /* ALternative casting permissions make static ability kind */
    let player_to_alternative_casting_permissions = [
        crate::ability_tree::terminals::PlayerSpecifier::You,
        crate::ability_tree::terminals::PlayerSpecifier::Any,
        crate::ability_tree::terminals::PlayerSpecifier::ToYourLeft,
        crate::ability_tree::terminals::PlayerSpecifier::ToYourRight,
        crate::ability_tree::terminals::PlayerSpecifier::EachOpponent,
    ]
    .into_iter()
    .map(|player| super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)).id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)).id(),
            ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)).id(),
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::From)).id(),
            ParserNode::ZoneReference { zone: dummy() }.id(),
        ]),
        merged: ParserNode::StaticAbilityKind { kind: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)),
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::From)),
                ParserNode::ZoneReference { zone },
            ] => Ok(ParserNode::StaticAbilityKind {
                kind: crate::ability_tree::ability::statik::StaticAbilityKind::AlternativeCastingPermissions(
                    crate::ability_tree::ability::statik::alterative_casting_permissions::AlternativeCastingPermissions {
                        player: player.clone(),
                        object: reference.clone(),
                        from_zone: zone.clone(),
                        additional_cost: None,
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    /* ALternative casting permissions make static ability kind */
    let player_to_alternative_casting_permissions_with_add_cost = [
        crate::ability_tree::terminals::PlayerSpecifier::You,
        crate::ability_tree::terminals::PlayerSpecifier::Any,
        crate::ability_tree::terminals::PlayerSpecifier::ToYourLeft,
        crate::ability_tree::terminals::PlayerSpecifier::ToYourRight,
        crate::ability_tree::terminals::PlayerSpecifier::EachOpponent,
    ]
    .into_iter()
    .map(|player| super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)).id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)).id(),
            ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)).id(),
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::From)).id(),
            ParserNode::ZoneReference { zone: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::By)).id(),
            ParserNode::Cost { cost: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::InAdditionToPayingItsOtherCost(
                non_terminals::InAdditionToPayingItsOtherCost,
            ))
            .id(),
        ]),
        merged: ParserNode::StaticAbilityKind { kind: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)),
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::From)),
                ParserNode::ZoneReference { zone },
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::By)),
                ParserNode::Cost { cost: inner },
                ParserNode::LexerToken(TokenKind::InAdditionToPayingItsOtherCost(non_terminals::InAdditionToPayingItsOtherCost)),
            ] => Ok(ParserNode::StaticAbilityKind {
                kind: crate::ability_tree::ability::statik::StaticAbilityKind::AlternativeCastingPermissions(
                    crate::ability_tree::ability::statik::alterative_casting_permissions::AlternativeCastingPermissions {
                        player: player.clone(),
                        object: reference.clone(),
                        from_zone: zone.clone(),
                        additional_cost: Some(inner.clone()),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    [
        static_ability_kind_rules,
        player_to_alternative_casting_permissions,
        player_to_alternative_casting_permissions_with_add_cost,
    ]
    .into_iter()
    .flatten()
}
