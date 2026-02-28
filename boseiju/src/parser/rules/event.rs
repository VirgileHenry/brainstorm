mod create_token_event_rules;
mod creature_action_event_rules;
mod enters_the_battlefield_rules;
mod life_gained_event_rules;
mod player_action_event_rules;
mod put_counter_on_permanent_event_rules;

use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* Players can be the source of events: "if a player would <event action>" */
    let player_to_event_source = [
        terminals::PlayerSpecifier::Any,
        terminals::PlayerSpecifier::ToYourLeft,
        terminals::PlayerSpecifier::ToYourRight,
        terminals::PlayerSpecifier::You,
    ]
    .into_iter()
    .map(|player_specifier| super::ParserRule {
        expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier)).id()]),
        merged: ParserNode::EventSource { source: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(TokenKind::PlayerSpecifier(player_specifier))] => Ok(ParserNode::EventSource {
                source: crate::ability_tree::event::source::EventSource::Player(*player_specifier),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let default_event_rules = vec![
        /* "an effect" is a common event source: "if an effect would <event action>" */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::An)).id(),
                ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Effect)).id(),
            ]),
            merged: ParserNode::EventSource { source: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::An)),
                    ParserNode::LexerToken(TokenKind::VhyToSortLater(non_terminals::VhyToSortLater::Effect)),
                ] => Ok(ParserNode::EventSource {
                    source: crate::ability_tree::event::source::EventSource::AnEffect(
                        crate::ability_tree::event::source::EffectEventSource,
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [
        player_to_event_source,
        default_event_rules,
        create_token_event_rules::rules().collect::<Vec<_>>(),
        creature_action_event_rules::rules().collect::<Vec<_>>(),
        enters_the_battlefield_rules::rules().collect::<Vec<_>>(),
        life_gained_event_rules::rules().collect::<Vec<_>>(),
        player_action_event_rules::rules().collect::<Vec<_>>(),
        put_counter_on_permanent_event_rules::rules().collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
}
