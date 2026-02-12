use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = super::ParserRule> {
    [
        /* Control specifiers are object specifiers */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ControlSpecifier(
                terminals::ControlSpecifier::YouControl,
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::ControlSpecifier(terminals::ControlSpecifier::YouControl))] => {
                    Some(ParserNode::ObjectSpecifier {
                        specifier: crate::ability_tree::object::ObjectSpecifier::Control(terminals::ControlSpecifier::YouControl),
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::ControlSpecifier(
                terminals::ControlSpecifier::YouDontControl,
            ))
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::ControlSpecifier(terminals::ControlSpecifier::YouDontControl))] => {
                    Some(ParserNode::ObjectSpecifier {
                        specifier: crate::ability_tree::object::ObjectSpecifier::Control(
                            terminals::ControlSpecifier::YouDontControl,
                        ),
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Cast specifiers can be object specifiers */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::You)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)).id(),
            ]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::You)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)),
                ] => Some(ParserNode::ObjectSpecifier {
                    specifier: crate::ability_tree::object::ObjectSpecifier::Cast(terminals::CastSpecifier::YouCast),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::EachOpponent)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)).id(),
            ]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::EachOpponent)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Cast)),
                ] => Some(ParserNode::ObjectSpecifier {
                    specifier: crate::ability_tree::object::ObjectSpecifier::Cast(terminals::CastSpecifier::YourOpponentsCast),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* In some cases, object kinds can be kind specifiers */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::ObjectKind {
                kind: DummyInit::dummy_init(),
            }
            .id()]),
            result: ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ObjectKind { kind }] => Some(ParserNode::ObjectSpecifier {
                    specifier: crate::ability_tree::object::ObjectSpecifier::Kind(*kind),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Object specifiers can be regrouped */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::ObjectSpecifier {
                specifier: DummyInit::dummy_init(),
            }
            .id()]),
            result: ParserNode::ObjectSpecifiers {
                specifiers: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ObjectSpecifier { specifier }] => Some(ParserNode::ObjectSpecifiers {
                    specifiers: crate::ability_tree::object::ObjectSpecifiers::Single(specifier.clone()),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Specifier 1 OR specifier 2 */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::ObjectSpecifier {
                    specifier: DummyInit::dummy_init(),
                }
                .id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Or)).id(),
                ParserNode::ObjectSpecifier {
                    specifier: DummyInit::dummy_init(),
                }
                .id(),
            ]),
            result: ParserNode::ObjectSpecifiers {
                specifiers: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectSpecifier { specifier: spec1 },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Or)),
                    ParserNode::ObjectSpecifier { specifier: spec2 },
                ] => Some(ParserNode::ObjectSpecifiers {
                    specifiers: {
                        let mut specifiers = arrayvec::ArrayVec::new();
                        specifiers.push(spec1.clone());
                        specifiers.push(spec2.clone());
                        crate::ability_tree::object::ObjectSpecifiers::Or(specifiers)
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Specifier 1 AND specifier 2 */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::ObjectSpecifier {
                    specifier: DummyInit::dummy_init(),
                }
                .id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)).id(),
                ParserNode::ObjectSpecifier {
                    specifier: DummyInit::dummy_init(),
                }
                .id(),
            ]),
            result: ParserNode::ObjectSpecifiers {
                specifiers: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectSpecifier { specifier: spec1 },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::And)),
                    ParserNode::ObjectSpecifier { specifier: spec2 },
                ] => Some(ParserNode::ObjectSpecifiers {
                    specifiers: {
                        let mut specifiers = arrayvec::ArrayVec::new();
                        specifiers.push(spec1.clone());
                        specifiers.push(spec2.clone());
                        crate::ability_tree::object::ObjectSpecifiers::And(specifiers)
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Or lists can be longer with: A, B, C and/or D. In that case, the separator is a comma */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::ObjectSpecifier {
                    specifier: DummyInit::dummy_init(),
                }
                .id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                ParserNode::ObjectSpecifiers {
                    specifiers: DummyInit::dummy_init(),
                }
                .id(),
            ]),
            result: ParserNode::ObjectSpecifiers {
                specifiers: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectSpecifier { specifier },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Some(ParserNode::ObjectSpecifiers {
                    specifiers: {
                        match specifiers {
                            crate::ability_tree::object::ObjectSpecifiers::Or(specifiers) => {
                                let mut new_specifiers = specifiers.clone();
                                new_specifiers.push(specifier.clone());
                                crate::ability_tree::object::ObjectSpecifiers::Or(new_specifiers)
                            }
                            crate::ability_tree::object::ObjectSpecifiers::And(specifiers) => {
                                let mut new_specifiers = specifiers.clone();
                                new_specifiers.push(specifier.clone());
                                crate::ability_tree::object::ObjectSpecifiers::And(new_specifiers)
                            }
                            crate::ability_tree::object::ObjectSpecifiers::Single(_) => return None,
                            crate::ability_tree::object::ObjectSpecifiers::OrOfAnd(_) => {
                                /* Here, it's important to reject the parsing, as we can't tell which specifier were distributed.
                                 * The parser must start by parsing the full comma array, then distribute. */
                                return None;
                            }
                        }
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Here, it gets more tricky.
         * If we have a specifier before / after an or specifier, it applies to both in an AndOfOr list */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::ObjectSpecifier {
                    specifier: DummyInit::dummy_init(),
                }
                .id(),
                ParserNode::ObjectSpecifiers {
                    specifiers: DummyInit::dummy_init(),
                }
                .id(),
            ]),
            result: ParserNode::ObjectSpecifiers {
                specifiers: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectSpecifier { specifier },
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Some(ParserNode::ObjectSpecifiers {
                    specifiers: {
                        match specifiers {
                            /* New specifier before and or: it's an "or of ands".
                             * For example, "basic plain or forest card".
                             * The "and" shall apply to all the previous specifiers */
                            crate::ability_tree::object::ObjectSpecifiers::Or(specifiers) => {
                                let mut new_specifiers = arrayvec::ArrayVec::new();
                                for prev_or_specifier in specifiers.iter() {
                                    let mut and_specifiers = arrayvec::ArrayVec::new();
                                    and_specifiers.push(specifier.clone());
                                    and_specifiers.push(prev_or_specifier.clone());
                                    new_specifiers.push(and_specifiers);
                                }
                                crate::ability_tree::object::ObjectSpecifiers::OrOfAnd(new_specifiers)
                            }
                            /* Here, we are just chaining ands: "red creature and artifact card" is all ands */
                            crate::ability_tree::object::ObjectSpecifiers::And(specifiers) => {
                                let mut new_specifiers = specifiers.clone();
                                new_specifiers.insert(0, specifier.clone());
                                crate::ability_tree::object::ObjectSpecifiers::And(new_specifiers)
                            }
                            /* A specifier in front of a single specifier is basically and and: "red creature" */
                            crate::ability_tree::object::ObjectSpecifiers::Single(prev_specifier) => {
                                let mut new_specifiers = arrayvec::ArrayVec::new();
                                new_specifiers.push(specifier.clone());
                                new_specifiers.push(prev_specifier.clone());
                                crate::ability_tree::object::ObjectSpecifiers::And(new_specifiers)
                            }
                            /* We already have an Or of Ands, add the specifier to the full array:
                             * "Red Artifact Creature or Enchantment" red shall be distributed to everything */
                            crate::ability_tree::object::ObjectSpecifiers::OrOfAnd(specifiers) => {
                                let mut new_specifiers = specifiers.clone();
                                for specifiers in new_specifiers.iter_mut() {
                                    specifiers.insert(0, specifier.clone());
                                }
                                crate::ability_tree::object::ObjectSpecifiers::OrOfAnd(new_specifiers)
                            }
                        }
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Same for specifiers after or lists, like in "instant or sorcery card" */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::ObjectSpecifiers {
                    specifiers: DummyInit::dummy_init(),
                }
                .id(),
                ParserNode::ObjectSpecifier {
                    specifier: DummyInit::dummy_init(),
                }
                .id(),
            ]),
            result: ParserNode::ObjectSpecifiers {
                specifiers: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectSpecifiers { specifiers },
                    ParserNode::ObjectSpecifier { specifier },
                ] => Some(ParserNode::ObjectSpecifiers {
                    specifiers: {
                        match specifiers {
                            /* New specifier after and or: it's an "or of ands".
                             * For example, "red or green creature".
                             * The "and" shall apply to all the previous specifiers */
                            crate::ability_tree::object::ObjectSpecifiers::Or(specifiers) => {
                                let mut new_specifiers = arrayvec::ArrayVec::new();
                                for prev_or_specifier in specifiers.iter() {
                                    let mut and_specifiers = arrayvec::ArrayVec::new();
                                    and_specifiers.push(prev_or_specifier.clone());
                                    and_specifiers.push(specifier.clone());
                                    new_specifiers.push(and_specifiers);
                                }
                                crate::ability_tree::object::ObjectSpecifiers::OrOfAnd(new_specifiers)
                            }
                            /* Fixme because maybe this will be a problem later on.
                             * When there is "A and B C", it usually means "any of A C or B C".
                             * For instance, "instants and sorceries you cast" -> applies to both, not spells that are both.
                             * So I'm making a OrOfAnd out of it for now.
                             */
                            crate::ability_tree::object::ObjectSpecifiers::And(specifiers) => {
                                let mut new_specifiers = arrayvec::ArrayVec::new();
                                for prev_or_specifier in specifiers.iter() {
                                    let mut and_specifiers = arrayvec::ArrayVec::new();
                                    and_specifiers.push(prev_or_specifier.clone());
                                    and_specifiers.push(specifier.clone());
                                    new_specifiers.push(and_specifiers);
                                }
                                crate::ability_tree::object::ObjectSpecifiers::OrOfAnd(new_specifiers)
                            }
                            /* A specifier after a single specifier is basically and and: "red creature" */
                            crate::ability_tree::object::ObjectSpecifiers::Single(prev_specifier) => {
                                let mut new_specifiers = arrayvec::ArrayVec::new();
                                new_specifiers.push(prev_specifier.clone());
                                new_specifiers.push(specifier.clone());
                                crate::ability_tree::object::ObjectSpecifiers::And(new_specifiers)
                            }
                            /* We already have an Or of Ands, add the specifier to the full array:
                             * "Red or Green Artifact Creature" creature shall be distributed to everything */
                            crate::ability_tree::object::ObjectSpecifiers::OrOfAnd(specifiers) => {
                                let mut new_specifiers = specifiers.clone();
                                for specifiers in new_specifiers.iter_mut() {
                                    specifiers.push(specifier.clone());
                                }
                                crate::ability_tree::object::ObjectSpecifiers::OrOfAnd(new_specifiers)
                            }
                        }
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
