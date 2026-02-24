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
    let put_specific_counter_events = terminals::Counter::all()
        .map(|counter| {
            [
                /* straight up "put a counter on a permanent" */
                ParserRule {
                    expanded: RuleLhs::new(&[
                        ParserNode::EventSource { source: dummy() }.id(),
                        ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)).id(),
                        ParserNode::Number { number: dummy() }.id(),
                        ParserNode::LexerToken(TokenKind::Counter(counter)).id(),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)).id(),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
                        ParserNode::ObjectReference { reference: dummy() }.id(),
                    ]),
                    merged: ParserNode::Event { event: dummy() }.id(),
                    reduction: |nodes: &[ParserNode]| match &nodes {
                        &[
                            ParserNode::EventSource { source },
                            ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)),
                            ParserNode::Number { number },
                            ParserNode::LexerToken(TokenKind::Counter(counter)),
                            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)),
                            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)),
                            ParserNode::ObjectReference { reference },
                        ] => Some(ParserNode::Event {
                            event: crate::ability_tree::event::Event::PutCounterOnPermanent(
                                crate::ability_tree::event::PutCounterOnPermanentEvent {
                                    source: source.clone(),
                                    quantity: number.clone(),
                                    counter_kind: Some(counter.clone()),
                                    on_permanent: reference.clone(),
                                },
                            ),
                        }),
                        _ => None,
                    },
                    creation_loc: ParserRuleDeclarationLocation::here(),
                },
                /* Would put counter on permanent */
                ParserRule {
                    expanded: RuleLhs::new(&[
                        ParserNode::EventSource { source: dummy() }.id(),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)).id(),
                        ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)).id(),
                        ParserNode::Number { number: dummy() }.id(),
                        ParserNode::LexerToken(TokenKind::Counter(counter)).id(),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)).id(),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
                        ParserNode::ObjectReference { reference: dummy() }.id(),
                    ]),
                    merged: ParserNode::Event { event: dummy() }.id(),
                    reduction: |nodes: &[ParserNode]| match &nodes {
                        &[
                            ParserNode::EventSource { source },
                            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)),
                            ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)),
                            ParserNode::Number { number },
                            ParserNode::LexerToken(TokenKind::Counter(counter)),
                            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)),
                            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)),
                            ParserNode::ObjectReference { reference },
                        ] => Some(ParserNode::Event {
                            event: crate::ability_tree::event::Event::PutCounterOnPermanent(
                                crate::ability_tree::event::PutCounterOnPermanentEvent {
                                    source: source.clone(),
                                    quantity: number.clone(),
                                    counter_kind: Some(counter.clone()),
                                    on_permanent: reference.clone(),
                                },
                            ),
                        }),
                        _ => None,
                    },
                    creation_loc: ParserRuleDeclarationLocation::here(),
                },
            ]
        })
        .flatten()
        .collect::<Vec<_>>();

    let put_any_counter_events = vec![
        /* straight up "put a counter on a permanent" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Counter)).id(), /* Fixme: counter is a verb here */
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Counter)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)),
                    ParserNode::ObjectReference { reference },
                ] => Some(ParserNode::Event {
                    event: crate::ability_tree::event::Event::PutCounterOnPermanent(
                        crate::ability_tree::event::PutCounterOnPermanentEvent {
                            source: source.clone(),
                            quantity: number.clone(),
                            counter_kind: None,
                            on_permanent: reference.clone(),
                        },
                    ),
                }),
                _ => None,
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Would put counter on permanent */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)).id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Counter)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)),
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Counter)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)),
                    ParserNode::ObjectReference { reference },
                ] => Some(ParserNode::Event {
                    event: crate::ability_tree::event::Event::PutCounterOnPermanent(
                        crate::ability_tree::event::PutCounterOnPermanentEvent {
                            source: source.clone(),
                            quantity: number.clone(),
                            counter_kind: None,
                            on_permanent: reference.clone(),
                        },
                    ),
                }),
                _ => None,
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [put_specific_counter_events, put_any_counter_events].into_iter().flatten()
}
