use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

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
                        source: crate::ability_tree::event::replacement::EventSourceReference::ThatEvent,
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
            result: ParserNode::CreatedTokenKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Of)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Those)),
                    ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::Supertype(
                        mtg_data::Supertype::Token,
                    ))),
                ] => Some(ParserNode::CreatedTokenKind {
                    kind: crate::ability_tree::imperative::CreatedTokenKind::PreviouslyMentionnedToken,
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
                ParserNode::CreatedTokenKind { kind: dummy() }.id(),
            ]),
            result: ParserNode::EventReplacement { replacement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSourceReference { source },
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)),
                    ParserNode::LexerToken(TokenKind::Number(non_terminals::Number::TwiceThatMany)),
                    ParserNode::CreatedTokenKind { kind },
                ] => {
                    use crate::ability_tree::event::replacement;
                    use crate::ability_tree::number::Number;
                    Some(ParserNode::EventReplacement {
                        replacement: replacement::EventReplacement::TokenCreation(replacement::TokenCreationReplacement {
                            source_ref: source.clone(),
                            create_tokens: crate::ability_tree::imperative::CreateTokenImperative {
                                tokens: {
                                    let mut tokens = arrayvec::ArrayVec::new_const();
                                    tokens.push(crate::ability_tree::imperative::TokenCreation {
                                        amount: Number::ThatMany,
                                        token: kind.clone(),
                                    });
                                    tokens.push(crate::ability_tree::imperative::TokenCreation {
                                        amount: Number::ThatMany,
                                        token: kind.clone(),
                                    });
                                    tokens
                                },
                            },
                        }),
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
            result: ParserNode::PutCounterKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Of)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Those)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Counter)),
                ] => Some(ParserNode::PutCounterKind {
                    kind: crate::ability_tree::imperative::CounterKind::PreviouslyMentionnedCounter,
                }),
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
                ParserNode::PutCounterKind { kind: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            result: ParserNode::EventReplacement { replacement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSourceReference { source },
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)),
                    ParserNode::LexerToken(TokenKind::Number(non_terminals::Number::TwiceThatMany)),
                    ParserNode::PutCounterKind { kind: counter_kind },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)),
                    ParserNode::ObjectReference {
                        reference: permanent_kind,
                    },
                ] => {
                    use crate::ability_tree::event::replacement;
                    use crate::ability_tree::number::Number;
                    Some(ParserNode::EventReplacement {
                        replacement: replacement::EventReplacement::CounterOnPermanent(
                            replacement::CounterOnPermanentReplacement {
                                source_ref: source.clone(),
                                put_counters: crate::ability_tree::imperative::PutCountersImperative {
                                    object: permanent_kind.clone(),
                                    counters: {
                                        let mut counters = arrayvec::ArrayVec::new_const();
                                        counters.push(crate::ability_tree::imperative::CounterOnPermanent {
                                            amount: Number::ThatMany,
                                            counter: counter_kind.clone(),
                                        });
                                        counters.push(crate::ability_tree::imperative::CounterOnPermanent {
                                            amount: Number::ThatMany,
                                            counter: counter_kind.clone(),
                                        });
                                        counters
                                    },
                                },
                            },
                        ),
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
