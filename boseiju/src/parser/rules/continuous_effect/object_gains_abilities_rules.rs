use crate::ability_tree::ability::statik::continuous_effect;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::lexer::tokens::intermediates::PowerToughnessModElements;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = ParserRule> {
    /* Object can "get" a power / toughness modifier */
    let power_toughness_modifier_rules = vec![
        /* Object gets straight +x/+x */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                    ParserNode::Number { number: toughness },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
                            continuous_effect::ContinuousEffectModifyObject {
                                object: reference.clone(),
                                modifications: {
                                    use continuous_effect::modify_object::PowerToughnessModifiers::PlusPlus;
                                    use continuous_effect::modify_object::*;

                                    let mut modifications = crate::utils::HeapArrayVec::new();
                                    let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                        ObjectCharacteristicModification::PowerToughnessModifiers(PlusPlus(
                                            PowerToughnessModifiersPlusPlus {
                                                power_mod: power.clone(),
                                                toughness_mod: toughness.clone(),
                                                #[cfg(feature = "spanned_tree")]
                                                span: span.merge(&toughness.node_span()),
                                            },
                                        )),
                                    );

                                    modifications.push(characteristic_mod);
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: reference.node_span().merge(&toughness.node_span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: reference.node_span().merge(&toughness.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Object gets +x/+x and has an ability */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AbilityKind { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                        #[cfg(feature = "spanned_tree")]
                            span: pt_mod_span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                    ParserNode::Number { number: toughness },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                        #[cfg(feature = "spanned_tree")]
                            span: gain_ab_span,
                    })),
                    ParserNode::AbilityKind { ability },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
                            continuous_effect::ContinuousEffectModifyObject {
                                object: reference.clone(),
                                modifications: {
                                    use continuous_effect::modify_object::PowerToughnessModifiers::PlusPlus;
                                    use continuous_effect::modify_object::*;

                                    let mut modifications = crate::utils::HeapArrayVec::new();
                                    let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                        ObjectCharacteristicModification::PowerToughnessModifiers(PlusPlus(
                                            PowerToughnessModifiersPlusPlus {
                                                power_mod: power.clone(),
                                                toughness_mod: toughness.clone(),
                                                #[cfg(feature = "spanned_tree")]
                                                span: pt_mod_span.merge(&toughness.node_span()),
                                            },
                                        )),
                                    );
                                    let gain_ab_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                        ability: ability.clone(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: gain_ab_span.merge(&ability.node_span()),
                                    });

                                    modifications.push(characteristic_mod);
                                    modifications.push(gain_ab_mod);
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: reference.node_span().merge(&ability.node_span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: reference.node_span().merge(&ability.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Object gets +x/+x and has an ability and another ability */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                #[cfg(feature = "spanned_tree")]
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AbilityKind { ability: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AbilityKind { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                        #[cfg(feature = "spanned_tree")]
                            span: pt_mod_span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                    ParserNode::Number { number: toughness },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                        #[cfg(feature = "spanned_tree")]
                            span: ab1_span,
                    })),
                    ParserNode::AbilityKind { ability: ability_1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::AbilityKind { ability: ability_2 },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
                            continuous_effect::ContinuousEffectModifyObject {
                                object: reference.clone(),
                                modifications: {
                                    use continuous_effect::modify_object::PowerToughnessModifiers::PlusPlus;
                                    use continuous_effect::modify_object::*;

                                    let mut modifications = crate::utils::HeapArrayVec::new();
                                    let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                        ObjectCharacteristicModification::PowerToughnessModifiers(PlusPlus(
                                            PowerToughnessModifiersPlusPlus {
                                                power_mod: power.clone(),
                                                toughness_mod: toughness.clone(),
                                                #[cfg(feature = "spanned_tree")]
                                                span: pt_mod_span.merge(&toughness.node_span()),
                                            },
                                        )),
                                    );
                                    let gain_ab1_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                        ability: ability_1.clone(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: ab1_span.merge(&ability_1.node_span()),
                                    });
                                    let gain_ab2_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                        ability: ability_2.clone(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: ability_2.node_span(),
                                    });

                                    modifications.push(characteristic_mod);
                                    modifications.push(gain_ab1_mod);
                                    modifications.push(gain_ab2_mod);
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: reference.node_span().merge(&ability_2.node_span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: reference.node_span().merge(&ability_2.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    /* Objects "have" abilities is a continuous effect. */
    let objects_have_abilities_only = vec![ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::AbilityKind { ability: dummy() }.id(),
        ]),
        merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                    #[cfg(feature = "spanned_tree")]
                    span,
                })),
                ParserNode::AbilityKind { ability },
            ] => Ok(ParserNode::ContinuousEffect {
                effect: continuous_effect::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
                        continuous_effect::ContinuousEffectModifyObject {
                            object: reference.clone(),
                            modifications: {
                                use continuous_effect::modify_object::*;

                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let gain_ab_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: ability.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.merge(&ability.node_span()),
                                });
                                modifications.push(gain_ab_mod);
                                modifications
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: reference.node_span().merge(&ability.node_span()),
                        },
                    ),
                    #[cfg(feature = "spanned_tree")]
                    span: reference.node_span().merge(&ability.node_span()),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }];

    /* Objects can "gain" abilities, but it means it is for some duration. */
    /* Here, the continuous effects are parsed straight into the imperative that generates them with a duration. */
    let object_gains_abilities_until = [
        crate::ability_tree::time::ForwardDuration::UntilEndOfTurn {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        crate::ability_tree::time::ForwardDuration::UntilEndOfYourNextTurn {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|duration|
        /* This rule does not use a continuous effect kind, as the "gain" word is only ever used in this context. */
        ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::AbilityKind { ability: dummy() }.id(),
            ParserNode::LexerToken(Token::ForwardDuration(duration)).id(),
        ]),
        merged: ParserNode::Imperative { imperative: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                    span,
                })),
                ParserNode::AbilityKind { ability },
                ParserNode::LexerToken(Token::ForwardDuration(duration)),
            ] => Ok(ParserNode::Imperative {
                imperative: crate::ability_tree::imperative::Imperative::GenerateContinuousEffect(
                    crate::ability_tree::imperative::GenerateContinuousEffectImperative {
                        effect: continuous_effect::ContinuousEffect {
                            effect: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
                                continuous_effect::ContinuousEffectModifyObject {
                                    object: reference.clone(),
                                    modifications: {
                                        use continuous_effect::modify_object::*;

                                        let mut modifications = crate::utils::HeapArrayVec::new();
                                        let gain_ab_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                            ability: ability.clone(),
                                            #[cfg(feature = "spanned_tree")]
                                            span: span.merge(&ability.node_span()),
                                        });
                                        modifications.push(gain_ab_mod);
                                        modifications
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: reference.node_span().merge(&ability.node_span()),
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: reference.node_span().merge(&duration.node_span()),
                        },
                        duration: duration.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: reference.node_span().merge(&duration.node_span()),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    [
        power_toughness_modifier_rules,
        objects_have_abilities_only,
        object_gains_abilities_until,
    ]
    .into_iter()
    .flatten()
}
