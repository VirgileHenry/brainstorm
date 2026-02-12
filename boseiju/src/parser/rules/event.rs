use super::ParserNode;
use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
use idris::Idris;

fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

pub fn rules() -> impl Iterator<Item = super::ParserRule> {
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

    let create_token_events = vec![
        /* Create token with no specifiers */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Supertype(
                    mtg_data::Supertype::Token,
                )))
                .id(),
            ]),
            result: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Supertype(mtg_data::Supertype::Token))),
                ] => Some(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreateTokens {
                        source: source.clone(),
                        quantity: number.clone(),
                        token_specifiers: None,
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Create token with special "under your control" specifier */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Supertype(
                    mtg_data::Supertype::Token,
                )))
                .id(),
                ParserNode::LexerToken(TokenKind::UnderControl(non_terminals::UnderControl::UnderYourControl)).id(),
            ]),
            result: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Supertype(mtg_data::Supertype::Token))),
                    ParserNode::LexerToken(TokenKind::UnderControl(non_terminals::UnderControl::UnderYourControl)),
                ] => Some(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreateTokens {
                        source: source.clone(),
                        quantity: number.clone(),
                        token_specifiers: Some(crate::ability_tree::object::ObjectSpecifiers::Single(
                            crate::ability_tree::object::ObjectSpecifier::Control(terminals::ControlSpecifier::YouControl),
                        )),
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Create token with specifiers before "token", as in "if you would create a treasure token" */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Supertype(
                    mtg_data::Supertype::Token,
                )))
                .id(),
            ]),
            result: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)),
                    ParserNode::Number { number },
                    ParserNode::ObjectSpecifiers { specifiers },
                    ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Supertype(mtg_data::Supertype::Token))),
                ] => Some(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreateTokens {
                        source: source.clone(),
                        quantity: number.clone(),
                        token_specifiers: Some(specifiers.clone()),
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    let put_specific_counter_events = terminals::Counter::all()
        .map(|counter| {
            [
                /* straight up "put a counter on a permanent" */
                super::ParserRule {
                    from: super::RuleLhs::new(&[
                        ParserNode::EventSource { source: dummy() }.id(),
                        ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)).id(),
                        ParserNode::Number { number: dummy() }.id(),
                        ParserNode::LexerToken(TokenKind::Counter(counter)).id(),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)).id(),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
                        ParserNode::ObjectReference { reference: dummy() }.id(),
                    ]),
                    result: ParserNode::Event { event: dummy() }.id(),
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
                            event: crate::ability_tree::event::Event::PutCounterOnPermanent {
                                source: source.clone(),
                                quantity: number.clone(),
                                counter_kind: Some(counter.clone()),
                                on_permanent: reference.clone(),
                            },
                        }),
                        _ => None,
                    },
                    creation_loc: super::ParserRuleDeclarationLocation::here(),
                },
                /* Would put counter on permanent */
                super::ParserRule {
                    from: super::RuleLhs::new(&[
                        ParserNode::EventSource { source: dummy() }.id(),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)).id(),
                        ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)).id(),
                        ParserNode::Number { number: dummy() }.id(),
                        ParserNode::LexerToken(TokenKind::Counter(counter)).id(),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)).id(),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
                        ParserNode::ObjectReference { reference: dummy() }.id(),
                    ]),
                    result: ParserNode::Event { event: dummy() }.id(),
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
                            event: crate::ability_tree::event::Event::PutCounterOnPermanent {
                                source: source.clone(),
                                quantity: number.clone(),
                                counter_kind: Some(counter.clone()),
                                on_permanent: reference.clone(),
                            },
                        }),
                        _ => None,
                    },
                    creation_loc: super::ParserRuleDeclarationLocation::here(),
                },
            ]
        })
        .flatten()
        .collect::<Vec<_>>();

    let put_any_counter_events = vec![
        /* straight up "put a counter on a permanent" */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Counter)).id(), /* Fixme: counter is a verb here */
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            result: ParserNode::Event { event: dummy() }.id(),
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
                    event: crate::ability_tree::event::Event::PutCounterOnPermanent {
                        source: source.clone(),
                        quantity: number.clone(),
                        counter_kind: None,
                        on_permanent: reference.clone(),
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Would put counter on permanent */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)).id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Counter)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::A)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            result: ParserNode::Event { event: dummy() }.id(),
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
                    event: crate::ability_tree::event::Event::PutCounterOnPermanent {
                        source: source.clone(),
                        quantity: number.clone(),
                        counter_kind: None,
                        on_permanent: reference.clone(),
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    let player_gains_life_event = [
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
            super::ParserRule {
                from: super::RuleLhs::new(&[
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
                        event: crate::ability_tree::event::Event::LifeGained {
                            player: *player_specifier,
                            minimum_amount: None,
                        },
                    }),
                    _ => None,
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* Past form: "if you have gained life" */
            super::ParserRule {
                from: super::RuleLhs::new(&[
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
                        event: crate::ability_tree::event::Event::LifeGained {
                            player: *player_specifier,
                            minimum_amount: None,
                        },
                    }),
                    _ => None,
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* Past form + number requirement: "if you have gained 3 or more life" */
            super::ParserRule {
                from: super::RuleLhs::new(&[
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
                        event: crate::ability_tree::event::Event::LifeGained {
                            player: *player_specifier,
                            minimum_amount: Some(number.clone()),
                        },
                    }),
                    _ => None,
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* Conditionnal form: "if you would gain life" */
            super::ParserRule {
                from: super::RuleLhs::new(&[
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
                        event: crate::ability_tree::event::Event::LifeGained {
                            player: *player_specifier,
                            minimum_amount: None,
                        },
                    }),
                    _ => None,
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
        ]
    })
    .flatten()
    .collect::<Vec<_>>();

    [
        player_to_event_source,
        default_event_rules,
        create_token_events,
        put_specific_counter_events,
        put_any_counter_events,
        player_gains_life_event,
    ]
    .into_iter()
    .flatten()
}
