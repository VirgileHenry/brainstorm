use super::ParserNode;
use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
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
                            span: count.span().merge(&specifiers.span()),
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
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Any {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::Target {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Any { span: any_span })),
                    ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::Target { span: target_span })),
                ] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: crate::ability_tree::object::CountSpecifier::Target(
                                crate::ability_tree::number::Number::Number(crate::ability_tree::number::FixedNumber {
                                    number: 1,
                                    span: *target_span,
                                }),
                            ),
                            specifiers: None,
                            span: any_span.merge(target_span),
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
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Other {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::Target {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Other { span: other_span })),
                    ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::Target { span: target_span })),
                ] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: crate::ability_tree::object::CountSpecifier::Target(number.clone()),
                            specifiers: Some(crate::ability_tree::object::ObjectSpecifiers::Single(
                                crate::ability_tree::object::ObjectSpecifier::NotPreviouslySelected(
                                    crate::ability_tree::object::NotPreviouslySelectedObjectSpecifier {
                                        span: other_span.merge(target_span),
                                    },
                                ),
                            )),
                            span: number.span().merge(target_span),
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
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Other {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::Target {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Other { span: other_span })),
                    ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::Target { .. })),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: crate::ability_tree::object::CountSpecifier::Target(number.clone()),
                            specifiers: {
                                let new_specifiers = specifiers.clone();
                                let another_specifier = crate::ability_tree::object::ObjectSpecifier::Another(
                                    crate::ability_tree::object::AnotherObjectSpecifier { span: *other_span },
                                );
                                Some(new_specifiers.add_factor_specifier(another_specifier))
                            },
                            span: number.span().merge(&specifiers.span()),
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
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::Target {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another { span: another_span })),
                    ParserNode::LexerToken(Token::CountSpecifier(intermediates::CountSpecifier::Target { span: target_span })),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: crate::ability_tree::object::CountSpecifier::Target(
                                crate::ability_tree::number::Number::Number(crate::ability_tree::number::FixedNumber {
                                    number: 1,
                                    span: *target_span,
                                }),
                            ),
                            specifiers: {
                                let new_specifiers = specifiers.clone();
                                let another_specifier = crate::ability_tree::object::ObjectSpecifier::Another(
                                    crate::ability_tree::object::AnotherObjectSpecifier { span: *another_span },
                                );
                                Some(new_specifiers.add_factor_specifier(another_specifier))
                            },
                            span: another_span.merge(&specifiers.span()),
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
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another { span: another_span })),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::ObjectReference {
                    reference: crate::ability_tree::object::ObjectReference::SpecifiedObj(
                        crate::ability_tree::object::SpecifiedObject {
                            amount: crate::ability_tree::object::CountSpecifier::A {
                                span: another_span.empty_at_start(),
                            },
                            specifiers: {
                                let new_specifiers = specifiers.clone();
                                let another_specifier = crate::ability_tree::object::ObjectSpecifier::Another(
                                    crate::ability_tree::object::AnotherObjectSpecifier { span: *another_span },
                                );
                                Some(new_specifiers.add_factor_specifier(another_specifier))
                            },
                            span: another_span.merge(&specifiers.span()),
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
                            amount: crate::ability_tree::object::CountSpecifier::All {
                                span: specifiers.span().empty_at_start(),
                            },
                            specifiers: Some(specifiers.clone()),
                            span: specifiers.span(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Self Referencing reference */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::SelfReferencing { reference: dummy() }).id()]),
            merged: ParserNode::ObjectReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::SelfReferencing { reference })] => Ok(ParserNode::ObjectReference {
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
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::PreviouslyMentionnedObject { object: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::It { span }))] => {
                    Ok(ParserNode::PreviouslyMentionnedObject {
                        object: crate::ability_tree::object::PreviouslyMentionnedObject { kind: None, span: *span },
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    /* Enchanted / equiped objects makes a special object specifier */
    let attached_objects_rules = [
        (
            terminals::PermanentState::Enchanted {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            object::CardType {
                card_type: mtg_data::CardType::Artifact,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ),
        (
            terminals::PermanentState::Enchanted {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            object::CardType {
                card_type: mtg_data::CardType::Creature,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ),
        (
            terminals::PermanentState::Enchanted {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            object::CardType {
                card_type: mtg_data::CardType::Enchantment,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ),
        (
            terminals::PermanentState::Enchanted {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            object::CardType {
                card_type: mtg_data::CardType::Land,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ),
        (
            terminals::PermanentState::Enchanted {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            object::CardType {
                card_type: mtg_data::CardType::Planeswalker,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ),
        (
            terminals::PermanentState::Equipped {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            object::CardType {
                card_type: mtg_data::CardType::Creature,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ),
    ]
    .into_iter()
    .map(|(state, card_type)| super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::LexerToken(Token::PermanentState(state)).id(),
            ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(
                card_type,
            )))
            .id(),
        ]),
        merged: ParserNode::ObjectReference { reference: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::PermanentState(state)),
                ParserNode::LexerToken(Token::ObjectKind(crate::ability_tree::object::ObjectKind::CardType(card_type))),
            ] => Ok(ParserNode::ObjectReference {
                reference: crate::ability_tree::object::ObjectReference::ObjectAttachedTo(
                    crate::ability_tree::object::ObjectAttachedTo {
                        span: state.span().merge(&card_type.span),
                    },
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
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::That {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ObjectKind(object_kind)).id(),
            ]),
            merged: ParserNode::PreviouslyMentionnedObject { object: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::That { span: that_span })),
                    ParserNode::LexerToken(Token::ObjectKind(object_kind)),
                ] => Ok(ParserNode::PreviouslyMentionnedObject {
                    object: crate::ability_tree::object::PreviouslyMentionnedObject {
                        kind: Some(object_kind.clone()),
                        span: that_span.merge(&object_kind.span()),
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
