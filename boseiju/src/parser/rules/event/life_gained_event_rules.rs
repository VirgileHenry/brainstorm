use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use idris::Idris;

fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        terminals::PlayerSpecifier::AnOpponent,
        terminals::PlayerSpecifier::TargetOpponent,
        terminals::PlayerSpecifier::ToYourLeft,
        terminals::PlayerSpecifier::ToYourRight,
        terminals::PlayerSpecifier::You,
    ]
    .into_iter()
    .map(|player_specifier| {
        [
            /* Present form: "whenever you gain life" */
            ParserRule {
                from: RuleLhs::new(&[
                    ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)).id(),
                    ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)).id(),
                    ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)).id(),
                ]),
                result: ParserNode::Event { event: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)),
                        ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)),
                        ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)),
                    ] => Some(ParserNode::Event {
                        event: crate::ability_tree::event::Event::LifeGained(crate::ability_tree::event::LifeGainedEvent {
                            player: *player_specifier,
                            minimum_amount: None,
                        }),
                    }),
                    _ => None,
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
            /* Past form: "if you have gained life" */
            ParserRule {
                from: RuleLhs::new(&[
                    ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)).id(),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)).id(),
                    ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)).id(),
                    ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)).id(),
                ]),
                result: ParserNode::Event { event: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)),
                        ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)),
                        ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)),
                    ] => Some(ParserNode::Event {
                        event: crate::ability_tree::event::Event::LifeGained(crate::ability_tree::event::LifeGainedEvent {
                            player: *player_specifier,
                            minimum_amount: None,
                        }),
                    }),
                    _ => None,
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
            /* Past form + number requirement: "if you have gained 3 or more life" */
            ParserRule {
                from: RuleLhs::new(&[
                    ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)).id(),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)).id(),
                    ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)).id(),
                    ParserNode::Number { number: dummy() }.id(),
                    ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)).id(),
                ]),
                result: ParserNode::Event { event: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Have)),
                        ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)),
                        ParserNode::Number { number },
                        ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)),
                    ] => Some(ParserNode::Event {
                        event: crate::ability_tree::event::Event::LifeGained(crate::ability_tree::event::LifeGainedEvent {
                            player: *player_specifier,
                            minimum_amount: Some(number.clone()),
                        }),
                    }),
                    _ => None,
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
            /* Conditionnal form: "if you would gain life" */
            ParserRule {
                from: RuleLhs::new(&[
                    ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)).id(),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)).id(),
                    ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)).id(),
                    ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)).id(),
                ]),
                result: ParserNode::Event { event: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)),
                        ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Gain)),
                        ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Life)),
                    ] => Some(ParserNode::Event {
                        event: crate::ability_tree::event::Event::LifeGained(crate::ability_tree::event::LifeGainedEvent {
                            player: *player_specifier,
                            minimum_amount: None,
                        }),
                    }),
                    _ => None,
                },
                creation_loc: ParserRuleDeclarationLocation::here(),
            },
        ]
    })
    .flatten()
}
