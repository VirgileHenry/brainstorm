use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* A count specifier as well as specifiers can be merged into an object reference */
    let count_and_object_to_ref = [
        terminals::CountSpecifier::All,
        terminals::CountSpecifier::Target,
        terminals::CountSpecifier::AnyNumberOfTargets,
        terminals::CountSpecifier::Others,
    ]
    .into_iter()
    .map(|count| super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::CountSpecifier(count)).id(),
            ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
        ]),
        merged: ParserNode::ObjectReference { reference: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::CountSpecifier(count)),
                ParserNode::ObjectSpecifiers { specifiers },
            ] => Some(ParserNode::ObjectReference {
                reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                    crate::ability_tree::object::SpecifiedObject {
                        amount: *count,
                        specifiers: specifiers.clone(),
                    },
                ),
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let object_references_rules = vec![
        /* "Another" is a special kind of specifier that is found before the "target" count specifier */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Another)).id(),
                ParserNode::LexerToken(TokenKind::CountSpecifier(terminals::CountSpecifier::Target)).id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Another)),
                    ParserNode::LexerToken(TokenKind::CountSpecifier(terminals::CountSpecifier::Target)),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Some(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: terminals::CountSpecifier::Target,
                            specifiers: {
                                let new_specifiers = specifiers.clone();
                                let another_specifier = crate::ability_tree::object::ObjectSpecifier::Another(
                                    crate::ability_tree::object::AnotherObjectSpecifier,
                                );
                                new_specifiers.add_factor_specifier(another_specifier)
                            },
                        },
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* In some cases, there will be no count specifier, there is an implicit "all". */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::ObjectSpecifiers { specifiers: dummy() }.id()]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ObjectSpecifiers { specifiers }] => Some(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: terminals::CountSpecifier::All,
                            specifiers: specifiers.clone(),
                        },
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Self Referencing reference */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::SelfReferencing { reference: dummy() }).id()]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::SelfReferencing { reference })] => Some(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SelfReferencing(reference.clone()),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Up to N + specifiers makes an object reference */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::CountSpecifier(terminals::CountSpecifier::UpTo { up_to: 0 })).id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::CountSpecifier(terminals::CountSpecifier::UpTo { up_to })),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Some(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: terminals::CountSpecifier::UpTo { up_to: *up_to },
                            specifiers: specifiers.clone(),
                        },
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::CountSpecifier(terminals::CountSpecifier::UpTo { up_to: 0 })).id(),
                ParserNode::LexerToken(TokenKind::CountSpecifier(terminals::CountSpecifier::Target)).id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::CountSpecifier(terminals::CountSpecifier::UpTo { up_to })),
                    ParserNode::LexerToken(TokenKind::CountSpecifier(terminals::CountSpecifier::Target)),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Some(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: terminals::CountSpecifier::UpTo { up_to: *up_to },
                            specifiers: specifiers.clone(),
                        },
                    ),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Previouslu mentionned objects can be object reference */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::PreviouslyMentionnedObject { object: dummy() }.id()]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::PreviouslyMentionnedObject { object }] => Some(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::PreviouslyMentionned(object.clone()),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    /* "That " + an object kind is a reference to previously mentionned object of that kind */
    let that_kind_to_previously_mentionned = crate::ability_tree::object::ObjectKind::all()
        .map(|object_kind| super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::That)).id(),
                ParserNode::LexerToken(TokenKind::ObjectKind(object_kind)).id(),
            ]),
            merged: ParserNode::PreviouslyMentionnedObject { object: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::That)),
                    ParserNode::LexerToken(TokenKind::ObjectKind(object_kind)),
                ] => Some(ParserNode::PreviouslyMentionnedObject {
                    object: crate::ability_tree::object::PreviouslyMentionnedObject {
                        kind: object_kind.clone(),
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    [
        count_and_object_to_ref,
        object_references_rules,
        that_kind_to_previously_mentionned,
    ]
    .into_iter()
    .flatten()
}
