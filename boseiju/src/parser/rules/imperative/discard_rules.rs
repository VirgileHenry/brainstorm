use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let any_player_discards_rules = [
        terminals::PlayerSpecifier::AnOpponent,
        terminals::PlayerSpecifier::TargetOpponent,
        terminals::PlayerSpecifier::EachOpponent,
        terminals::PlayerSpecifier::All,
        terminals::PlayerSpecifier::ToYourLeft,
        terminals::PlayerSpecifier::ToYourRight,
        terminals::PlayerSpecifier::You,
    ]
    .into_iter()
    .map(|player| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)).id(),
            ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Discard)).id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Card)).id(),
        ]),
        merged: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Discard)),
                ParserNode::Number { number },
                ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Card)),
            ] => Ok(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::Discard(
                    crate::ability_tree::imperative::DiscardImperative {
                        player: player.clone(),
                        amount: number.clone(),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let you_discard_rules = vec![
        /* "discard X" is a discard imperative for the owner of the effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Discard)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Card)).id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Discard)),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Card)),
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::Discard(
                        crate::ability_tree::imperative::DiscardImperative {
                            player: terminals::PlayerSpecifier::You,
                            amount: number.clone(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [any_player_discards_rules, you_discard_rules].into_iter().flatten()
}
