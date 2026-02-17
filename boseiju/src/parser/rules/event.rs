mod create_token_event_rules;
mod enters_the_battlefield_rules;
mod life_gained_event_rules;
mod player_casts_spell_rules;
mod put_counter_on_permanent_rules;

use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
use idris::Idris;

fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* Players can be the source of events: "if a player would <event action>" */
    let player_to_event_source = [
        terminals::PlayerSpecifier::AnOpponent,
        terminals::PlayerSpecifier::Any,
        terminals::PlayerSpecifier::ToYourLeft,
        terminals::PlayerSpecifier::ToYourRight,
        terminals::PlayerSpecifier::You,
    ]
    .into_iter()
    .map(|player_specifier| super::ParserRule {
        from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)).id()]),
        result: ParserNode::EventSource { source: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier))] => Some(ParserNode::EventSource {
                source: crate::ability_tree::event::source::EventSource::Player(*player_specifier),
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let default_event_rules = vec![
        /* "an effect" is a common event source: "if an effect would <event action>" */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::An)).id(),
                ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Effect)).id(),
            ]),
            result: ParserNode::EventSource { source: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::An)),
                    ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Effect)),
                ] => Some(ParserNode::EventSource {
                    source: crate::ability_tree::event::source::EventSource::AnEffect,
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [
        player_to_event_source,
        default_event_rules,
        create_token_event_rules::rules().collect::<Vec<_>>(),
        life_gained_event_rules::rules().collect::<Vec<_>>(),
        player_casts_spell_rules::rules().collect::<Vec<_>>(),
        put_counter_on_permanent_rules::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
