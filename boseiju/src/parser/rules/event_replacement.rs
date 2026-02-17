use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
use idris::Idris;

fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let default_event_replacement_rules = vec![
        /* Create source reference: that effect (it), that player, etc */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::It)).id(),
            ]),
            result: ParserNode::EventSourceReference { source: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::It))] => {
                    Some(ParserNode::EventSourceReference {
                        source: crate::ability_tree::event::replacement::source_ref::EventSourceReference::ThatEvent,
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    let token_creation_replacement_rules = vec![
        /* "Of those tokens" is a reference to previously mentionned tokens */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Of)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Those)).id(),
                ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::Supertype(
                    mtg_data::Supertype::Token,
                )))
                .id(),
            ]),
            result: ParserNode::ReplacedTokenKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Of)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Those)),
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::Supertype(
                        mtg_data::Supertype::Token,
                    ))),
                ] => Some(ParserNode::ReplacedTokenKind {
                    kind: crate::ability_tree::event::replacement::token_creation::ReplacedTokenKind::PreviouslyMentionnedToken,
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* token creation replacement with "twice that many" */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::EventSourceReference { source: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)).id(),
                ParserNode::LexerToken(TokenKind::Number(non_terminals::Number::TwiceThatMany)).id(),
                ParserNode::ReplacedTokenKind { kind: dummy() }.id(),
            ]),
            result: ParserNode::EventReplacement { replacement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSourceReference { source },
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)),
                    ParserNode::LexerToken(TokenKind::Number(non_terminals::Number::TwiceThatMany)),
                    ParserNode::ReplacedTokenKind { kind },
                ] => {
                    use crate::ability_tree::event::replacement;
                    use crate::ability_tree::number::Number;
                    Some(ParserNode::EventReplacement {
                        replacement: replacement::EventReplacement::TokenCreationReplacement {
                            source_ref: source.clone(),
                            tokens: vec![
                                replacement::token_creation::TokenCreation {
                                    amount: Number::ThatMany,
                                    token: kind.clone(),
                                },
                                replacement::token_creation::TokenCreation {
                                    amount: Number::ThatMany,
                                    token: kind.clone(),
                                },
                            ],
                        },
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    let put_counter_on_permanent_replacement_rules = vec![
        /* "Of those counters" is a reference to previously mentionned counters */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Of)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Those)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Counter)).id(), /* Fixme */
            ]),
            result: ParserNode::ReplacedCounterKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Of)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Those)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Counter)),
                ] => {
                    use crate::ability_tree::event::replacement::counter_on_permanent;
                    Some(ParserNode::ReplacedCounterKind {
                        kind: counter_on_permanent::ReplacedCounterKind::PreviouslyMentionnedCounter,
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* counter replacement with "twice that many" */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::EventSourceReference { source: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)).id(),
                ParserNode::LexerToken(TokenKind::Number(non_terminals::Number::TwiceThatMany)).id(),
                ParserNode::ReplacedCounterKind { kind: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            result: ParserNode::EventReplacement { replacement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSourceReference { source },
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)),
                    ParserNode::LexerToken(TokenKind::Number(non_terminals::Number::TwiceThatMany)),
                    ParserNode::ReplacedCounterKind { kind: counter_kind },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)),
                    ParserNode::ObjectReference {
                        reference: permanent_kind,
                    },
                ] => {
                    use crate::ability_tree::event::replacement;
                    use crate::ability_tree::number::Number;
                    Some(ParserNode::EventReplacement {
                        replacement: replacement::EventReplacement::CounterOnPermanentReplacement {
                            source_ref: source.clone(),
                            counters: vec![
                                replacement::counter_on_permanent::CounterOnPermanent {
                                    amount: Number::ThatMany,
                                    counter: counter_kind.clone(),
                                    on_permanent: permanent_kind.clone(),
                                },
                                replacement::counter_on_permanent::CounterOnPermanent {
                                    amount: Number::ThatMany,
                                    counter: counter_kind.clone(),
                                    on_permanent: permanent_kind.clone(),
                                },
                            ],
                        },
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [
        default_event_replacement_rules,
        token_creation_replacement_rules,
        put_counter_on_permanent_replacement_rules,
    ]
    .into_iter()
    .flatten()
}
