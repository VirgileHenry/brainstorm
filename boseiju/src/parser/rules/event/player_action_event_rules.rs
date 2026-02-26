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
    let player_attacks_event = [
        terminals::PlayerSpecifier::Any,
        terminals::PlayerSpecifier::AnOpponent,
        terminals::PlayerSpecifier::ToYourLeft,
        terminals::PlayerSpecifier::ToYourRight,
        terminals::PlayerSpecifier::You,
    ]
    .into_iter()
    .map(|player| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)).id(),
            ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Attack)).id(),
        ]),
        merged: ParserNode::Event { event: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)),
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Attack)),
            ] => Ok(ParserNode::Event {
                event: crate::ability_tree::event::Event::PlayerAction(crate::ability_tree::event::PlayerActionEvent {
                    player: player.clone(),
                    action: crate::ability_tree::event::PlayerAction::Attacks(crate::ability_tree::event::PlayerAttacksAction {
                        attacked_player: None,
                        with: None,
                    }),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    [player_attacks_event].into_iter().flatten()
}
