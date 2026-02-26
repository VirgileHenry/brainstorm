use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let object_references_rules = vec![
        /* From an object count and specifiers, we can make an object reference */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CountSpecifier { count },
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: count.clone(),
                            specifiers: specifiers.clone(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "other" is a special kind of specifier that can be found between the number and the target. */
        /* This rule is kept from the "other" being simply a specifier,
         * since it's the only specifier that can be found between the number and the "target" */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Other)).id(),
                ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::Target)).id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Other)),
                    ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::Target)),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: crate::ability_tree::object::CountSpecifier::Target(number.clone()),
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
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "Another" is the same as "1 other", see the comment above */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Another)).id(),
                ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::Target)).id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Another)),
                    ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::Target)),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: crate::ability_tree::object::CountSpecifier::Target(
                                crate::ability_tree::number::Number::Number(crate::ability_tree::number::FixedNumber {
                                    number: 1,
                                }),
                            ),
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
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* In some cases, there will be no count specifier, there is an implicit "all". */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::ObjectSpecifiers { specifiers: dummy() }.id()]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ObjectSpecifiers { specifiers }] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: crate::ability_tree::object::CountSpecifier::All,
                            specifiers: specifiers.clone(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Self Referencing reference */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::SelfReferencing { reference: dummy() }).id()]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::SelfReferencing { reference })] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SelfReferencing(reference.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Previously mentionned objects can be object reference */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::PreviouslyMentionnedObject { object: dummy() }.id()]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::PreviouslyMentionnedObject { object }] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::PreviouslyMentionned(object.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "it" can be a previously mentionned object */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::EnglishKeyword(
                non_terminals::EnglishKeyword::It,
            ))
            .id()]),
            merged: ParserNode::PreviouslyMentionnedObject { object: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::It))] => {
                    Ok(ParserNode::PreviouslyMentionnedObject {
                        object: crate::ability_tree::object::PreviouslyMentionnedObject { kind: None },
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
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
                ] => Ok(ParserNode::PreviouslyMentionnedObject {
                    object: crate::ability_tree::object::PreviouslyMentionnedObject {
                        kind: Some(object_kind.clone()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    [object_references_rules, that_kind_to_previously_mentionned]
        .into_iter()
        .flatten()
}
