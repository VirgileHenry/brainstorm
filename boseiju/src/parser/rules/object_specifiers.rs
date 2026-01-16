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
                        }
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* when there is no clear separator, it's an "and". for example, "black creatures you control" are all 3 specifiers. */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::ObjectSpecifier {
                    specifier: DummyInit::dummy_init(),
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
                    ParserNode::ObjectSpecifier { specifier: spec1 },
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
    ]
    .into_iter()
}
