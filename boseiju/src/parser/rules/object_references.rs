use super::ParserNode;
use crate::ability_tree::terminals;
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
                            specifiers: Some(specifiers.clone()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "any target" is special, as it has no object specifiers after it. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Any)).id(),
                ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::Target)).id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Any)),
                    ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::Target)),
                ] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: crate::ability_tree::object::CountSpecifier::Target(
                                crate::ability_tree::number::Number::Number(crate::ability_tree::number::FixedNumber {
                                    number: 1,
                                }),
                            ),
                            specifiers: None,
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "X other targets" on its own is kinda special, since it means "that has not already been targetted". */
        /* This must be parsed as an entire object reference, since adding specifiers will make this rule invalid */
        /* Fixme: this may cause problems later on */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Other)).id(),
                ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::Target)).id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Other)),
                    ParserNode::LexerToken(TokenKind::CountSpecifier(non_terminals::CountSpecifier::Target)),
                ] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: crate::ability_tree::object::CountSpecifier::Target(number.clone()),
                            specifiers: Some(crate::ability_tree::object::ObjectSpecifiers::Single(
                                crate::ability_tree::object::ObjectSpecifier::NotPreviouslySelected(
                                    crate::ability_tree::object::NotPreviouslySelectedObjectSpecifier,
                                ),
                            )),
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
                                Some(new_specifiers.add_factor_specifier(another_specifier))
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
                                Some(new_specifiers.add_factor_specifier(another_specifier))
                            },
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "Another <specifiers>" bypasses the count specifier, since it's "a other" */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Another)).id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Another)),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: crate::ability_tree::object::CountSpecifier::A,
                            specifiers: {
                                let new_specifiers = specifiers.clone();
                                let another_specifier = crate::ability_tree::object::ObjectSpecifier::Another(
                                    crate::ability_tree::object::AnotherObjectSpecifier,
                                );
                                Some(new_specifiers.add_factor_specifier(another_specifier))
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
                            specifiers: Some(specifiers.clone()),
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

    /* Enchanted / equiped objects makes a special object specifier */
    let attached_objects_rules = [
        (terminals::PermanentState::Enchanted, mtg_data::CardType::Artifact),
        (terminals::PermanentState::Enchanted, mtg_data::CardType::Creature),
        (terminals::PermanentState::Enchanted, mtg_data::CardType::Enchantment),
        (terminals::PermanentState::Enchanted, mtg_data::CardType::Land),
        (terminals::PermanentState::Enchanted, mtg_data::CardType::Planeswalker),
        (terminals::PermanentState::Equipped, mtg_data::CardType::Creature),
    ]
    .into_iter()
    .map(|(state, card_type)| super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PermanentState(state)).id(),
            ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                card_type,
            )))
            .id(),
        ]),
        merged: ParserNode::ObjectReference { reference: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PermanentState(_)),
                ParserNode::LexerToken(TokenKind::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(_))),
            ] => Ok(ParserNode::ObjectReference {
                reference: crate::ability_tree::object::ObjectReference::ObjectAttachedTo(
                    crate::ability_tree::object::ObjectAttachedTo,
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

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

    [
        object_references_rules,
        attached_objects_rules,
        that_kind_to_previously_mentionned,
    ]
    .into_iter()
    .flatten()
}
