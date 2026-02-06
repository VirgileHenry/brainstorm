use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::parser::node::DummyInit;
use idris::Idris;

fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

pub fn rules() -> impl Iterator<Item = super::ParserRule> {
    let player_did_action = [
        terminals::PlayerSpecifier::AnOpponent,
        terminals::PlayerSpecifier::All,
        terminals::PlayerSpecifier::ToYourLeft,
        terminals::PlayerSpecifier::ToYourRight,
        terminals::PlayerSpecifier::You,
    ]
    .into_iter()
    .map(|player_specifier| super::ParserRule {
        from: super::RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)).id(),
            ParserNode::PlayerAction { player_action: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::ThisTurn).id(),
        ]),
        result: ParserNode::IfCondition { condition: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)),
                ParserNode::PlayerAction { player_action },
                ParserNode::LexerToken(TokenKind::ThisTurn),
            ] => Some(ParserNode::IfCondition {
                condition: crate::ability_tree::if_condition::IfCondition::PlayerDidAction {
                    player: *player_specifier,
                    action: player_action.clone(),
                    timeframe: crate::ability_tree::if_condition::condition_timeframe::ConditionTimeframe::ThisTurn,
                },
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    });
    [player_did_action].into_iter().flatten()
}
