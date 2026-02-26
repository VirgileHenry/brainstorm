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
    /* Put counters on something, "put 2 +1/+1 counters on each creature you control" */
    let remove_counters_rules = terminals::Counter::all()
        .map(|counter| ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Remove)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::Counter(counter)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::From)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Among)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Remove)),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::Counter(counter)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::From)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Among)),
                    ParserNode::ObjectReference { reference },
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::RemoveCounters(
                        crate::ability_tree::imperative::RemoveCountersImperative {
                            object: reference.clone(),
                            counters: {
                                let mut counters = arrayvec::ArrayVec::new();
                                counters.push(crate::ability_tree::imperative::RemovableCounterOnPermanent {
                                    amount: number.clone(),
                                    counter: crate::ability_tree::imperative::RemovableCounterKind::NewCounter(counter.clone()),
                                });
                                counters
                            },
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let remove_any_counter_rules = vec![ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Remove)).id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Counter)).id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::From)).id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Among)).id(),
            ParserNode::ObjectReference { reference: dummy() }.id(),
        ]),
        merged: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PlayerAction(non_terminals::PlayerAction::Remove)),
                ParserNode::Number { number },
                ParserNode::LexerToken(TokenKind::AmbiguousToken(non_terminals::AmbiguousToken::Counter)),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::From)),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Among)),
                ParserNode::ObjectReference { reference },
            ] => Ok(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::RemoveCounters(
                    crate::ability_tree::imperative::RemoveCountersImperative {
                        object: reference.clone(),
                        counters: {
                            let mut counters = arrayvec::ArrayVec::new();
                            counters.push(crate::ability_tree::imperative::RemovableCounterOnPermanent {
                                amount: number.clone(),
                                counter: crate::ability_tree::imperative::RemovableCounterKind::AnyCounter,
                            });
                            counters
                        },
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }];

    [remove_counters_rules, remove_any_counter_rules].into_iter().flatten()
}
