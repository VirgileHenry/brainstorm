mod create_token_event_rules;
mod creature_action_event_rules;
mod enters_the_battlefield_rules;
mod life_gained_event_rules;
mod player_action_event_rules;
mod put_counter_on_permanent_event_rules;

use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* Players can be the source of events: "if a player would <event action>" */
    let player_to_event_source = [
        terminals::PlayerSpecifier::Any {
            span: Default::default(),
        },
        terminals::PlayerSpecifier::ToYourLeft {
            span: Default::default(),
        },
        terminals::PlayerSpecifier::ToYourRight {
            span: Default::default(),
        },
        terminals::PlayerSpecifier::You {
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|player_specifier| super::ParserRule {
        expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::PlayerSpecifier(player_specifier)).id()]),
        merged: ParserNode::EventSource { source: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(Token::PlayerSpecifier(player_specifier))] => Ok(ParserNode::EventSource {
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
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::An {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Effect {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::EventSource { source: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::An { span: start_span })),
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Effect { span: end_span })),
                ] => Ok(ParserNode::EventSource {
                    source: crate::ability_tree::event::source::EventSource::AnEffect(
                        crate::ability_tree::event::source::EffectEventSource {
                            span: start_span.merge(end_span),
                        },
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
