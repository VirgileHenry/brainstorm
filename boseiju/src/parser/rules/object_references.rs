use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = super::ParserRule> {
    /* A count specifier as well as specifiers can be merged into an object reference */
    let count_and_object_to_ref = [
        terminals::CountSpecifier::All,
        terminals::CountSpecifier::Target,
        terminals::CountSpecifier::AnyNumberOfTargets,
        terminals::CountSpecifier::Others,
    ]
    .into_iter()
    .map(|count| super::ParserRule {
        from: super::RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::CountSpecifier(count)).id(),
            ParserNode::ObjectSpecifiers {
                specifiers: DummyInit::dummy_init(),
            }
            .id(),
        ]),
        result: ParserNode::ObjectReference {
            reference: DummyInit::dummy_init(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::CountSpecifier(count)),
                ParserNode::ObjectSpecifiers { specifiers },
            ] => Some(ParserNode::ObjectReference {
                reference: crate::ability_tree::object::ObjectReference::SpecifiedObj {
                    amount: *count,
                    specifiers: specifiers.clone(),
                },
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let non_repeting_rules = vec![
        /* Some cases, there will be no count specifier, there is an implicit "all". */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::ObjectSpecifiers {
                specifiers: DummyInit::dummy_init(),
            }
            .id()]),
            result: ParserNode::ObjectReference {
                reference: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ObjectSpecifiers { specifiers }] => Some(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj {
                        amount: terminals::CountSpecifier::All,
                        specifiers: specifiers.clone(),
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Self Referencing reference */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::SelfReferencing {
                reference: non_terminals::SelfReferencing,
            })
            .id()]),
            result: ParserNode::ObjectReference {
                reference: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::SelfReferencing {
                        reference: non_terminals::SelfReferencing,
                    }),
                ] => Some(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SelfReferencing,
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Up to N + specifiers makes an object reference */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::CountSpecifier(terminals::CountSpecifier::UpTo { up_to: 0 })).id(),
                ParserNode::ObjectSpecifiers {
                    specifiers: DummyInit::dummy_init(),
                }
                .id(),
            ]),
            result: ParserNode::ObjectReference {
                reference: DummyInit::dummy_init(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::CountSpecifier(terminals::CountSpecifier::UpTo { up_to })),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Some(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj {
                        amount: terminals::CountSpecifier::UpTo { up_to: *up_to },
                        specifiers: specifiers.clone(),
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [count_and_object_to_ref, non_repeting_rules].into_iter().flatten()
}
