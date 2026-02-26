use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let any_player_draws_rules = [
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
            ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Draw)).id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Card)).id(),
        ]),
        merged: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)),
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Draw)),
                ParserNode::Number { number },
                ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Card)),
            ] => Ok(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::Draw(crate::ability_tree::imperative::DrawImperative {
                    player: player.clone(),
                    amount: number.clone(),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let you_draw_rules = vec![
        /* "draw X" is a draw imperative for the owner of the effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Draw)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Card)).id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Draw)),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Card)),
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::Draw(
                        crate::ability_tree::imperative::DrawImperative {
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

    [any_player_draws_rules, you_draw_rules].into_iter().flatten()
}
