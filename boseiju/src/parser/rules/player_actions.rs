use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
use idris::Idris;

fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

pub fn rules() -> impl Iterator<Item = super::ParserRule> {
    [
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)).id(),
                ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)).id(),
            ]),
            result: ParserNode::PlayerAction { player_action: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)),
                    ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)),
                ] => Some(ParserNode::PlayerAction {
                    player_action: crate::ability_tree::player_action::PlayerAction::GainLife { minimum_amount: None },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)).id(),
                ParserNode::LexerToken(TokenKind::Number(terminals::Number::OrMore { num: 0 })).id(),
                ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)).id(),
            ]),
            result: ParserNode::PlayerAction { player_action: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)),
                    ParserNode::LexerToken(TokenKind::Number(terminals::Number::OrMore { num })),
                    ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)),
                ] => Some(ParserNode::PlayerAction {
                    player_action: crate::ability_tree::player_action::PlayerAction::GainLife {
                        minimum_amount: Some(*num),
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Fixme: maybe we will need to split the present / past tense at some point ? */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)).id(),
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)).id(),
                ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)).id(),
            ]),
            result: ParserNode::PlayerAction { player_action: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)),
                    ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)),
                    ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)),
                ] => Some(ParserNode::PlayerAction {
                    player_action: crate::ability_tree::player_action::PlayerAction::GainLife { minimum_amount: None },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)).id(),
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)).id(),
                ParserNode::LexerToken(TokenKind::Number(terminals::Number::OrMore { num: 0 })).id(),
                ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)).id(),
            ]),
            result: ParserNode::PlayerAction { player_action: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)),
                    ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)),
                    ParserNode::LexerToken(TokenKind::Number(terminals::Number::OrMore { num })),
                    ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)),
                ] => Some(ParserNode::PlayerAction {
                    player_action: crate::ability_tree::player_action::PlayerAction::GainLife {
                        minimum_amount: Some(*num),
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
