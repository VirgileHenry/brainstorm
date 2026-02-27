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
    let put_counters_rules = terminals::Counter::all()
        .map(|counter| ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::Counter(counter)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Put)),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::Counter(counter)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::On)),
                    ParserNode::ObjectReference { reference },
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::PutCounters(
                        crate::ability_tree::imperative::PutCountersImperative {
                            object: reference.clone(),
                            counters: {
                                let mut counters = crate::utils::HeapArrayVec::new();
                                counters.push(crate::ability_tree::imperative::CounterOnPermanent {
                                    amount: number.clone(),
                                    counter: crate::ability_tree::imperative::CounterKind::NewCounter(counter.clone()),
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

    [put_counters_rules].into_iter().flatten()
}
