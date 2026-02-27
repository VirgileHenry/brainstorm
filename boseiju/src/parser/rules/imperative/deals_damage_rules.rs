use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Some object deals damage to some other object */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Deals)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::DamageKind(non_terminals::DamageKind::Damage)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference: dealer },
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Deals)),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::DamageKind(non_terminals::DamageKind::Damage)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)),
                    ParserNode::ObjectReference { reference: to },
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::DealsDamage(
                        crate::ability_tree::imperative::DealsDamageImperative {
                            dealer: dealer.clone(),
                            damages: {
                                let mut damages = crate::utils::HeapArrayVec::new();
                                damages.push(crate::ability_tree::imperative::DamagesDealt {
                                    to: to.clone(),
                                    amount: number.clone(),
                                });
                                damages
                            },
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Sometimes, objects have two damage actions */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Deals)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::DamageKind(non_terminals::DamageKind::Damage)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::DamageKind(non_terminals::DamageKind::Damage)).id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)).id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference: dealer },
                    ParserNode::LexerToken(TokenKind::ActionKeyword(non_terminals::ActionKeyword::Deals)),
                    ParserNode::Number { number: num_d1 },
                    ParserNode::LexerToken(TokenKind::DamageKind(non_terminals::DamageKind::Damage)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)),
                    ParserNode::ObjectReference { reference: to_d1 },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)),
                    ParserNode::Number { number: num_d2 },
                    ParserNode::LexerToken(TokenKind::DamageKind(non_terminals::DamageKind::Damage)),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::To)),
                    ParserNode::ObjectReference { reference: to_d2 },
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::DealsDamage(
                        crate::ability_tree::imperative::DealsDamageImperative {
                            dealer: dealer.clone(),
                            damages: {
                                let mut damages = crate::utils::HeapArrayVec::new();
                                damages.push(crate::ability_tree::imperative::DamagesDealt {
                                    to: to_d1.clone(),
                                    amount: num_d1.clone(),
                                });
                                damages.push(crate::ability_tree::imperative::DamagesDealt {
                                    to: to_d2.clone(),
                                    amount: num_d2.clone(),
                                });
                                damages
                            },
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
