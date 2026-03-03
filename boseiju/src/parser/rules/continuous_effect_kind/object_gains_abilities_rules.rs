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

pub fn rules() -> impl Iterator<Item = ParserRule> {
    /* Object can "get" a power / toughness modifier */
    let power_toughness_modifier_rules = vec![
        /* Object gets straight +x/+x */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { span })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                    ParserNode::Number { number: toughness },
                ] => Ok(ParserNode::ContinuousEffectKind {
                    kind: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
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
                                            span: span.merge(&toughness.span()),
                                        },
                                    )),
                                );

                                modifications.push(characteristic_mod);
                                modifications
                            },
                            span: reference.span().merge(&toughness.span()),
                        },
                    ),
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
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AbilityKind { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                        span: pt_mod_span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                    ParserNode::Number { number: toughness },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have { span: gain_ab_span })),
                    ParserNode::AbilityKind { ability },
                ] => Ok(ParserNode::ContinuousEffectKind {
                    kind: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
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
                                            span: pt_mod_span.merge(&toughness.span()),
                                        },
                                    )),
                                );
                                let gain_ab_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: ability.clone(),
                                    span: gain_ab_span.merge(&ability.span()),
                                });

                                modifications.push(characteristic_mod);
                                modifications.push(gain_ab_mod);
                                modifications
                            },
                            span: reference.span().merge(&ability.span()),
                        },
                    ),
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
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AbilityKind { ability: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::AbilityKind { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                        span: pt_mod_span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                    ParserNode::Number { number: toughness },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have { span: ab1_span })),
                    ParserNode::AbilityKind { ability: ability_1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::AbilityKind { ability: ability_2 },
                ] => Ok(ParserNode::ContinuousEffectKind {
                    kind: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
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
                                            span: pt_mod_span.merge(&toughness.span()),
                                        },
                                    )),
                                );
                                let gain_ab1_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: ability_1.clone(),
                                    span: ab1_span.merge(&ability_1.span()),
                                });
                                let gain_ab2_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: ability_2.clone(),
                                    span: ability_2.span(),
                                });

                                modifications.push(characteristic_mod);
                                modifications.push(gain_ab1_mod);
                                modifications.push(gain_ab2_mod);
                                modifications
                            },
                            span: reference.span().merge(&ability_2.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    /* Objects "have" abilities is a continuous effect kind. */
    /* "have" means its static, we shall parse it directly */
    let objects_have_abilities_only = vec![ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                span: Default::default(),
            }))
            .id(),
            ParserNode::AbilityKind { ability: dummy() }.id(),
        ]),
        merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have { span })),
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
                                    span: span.merge(&ability.span()),
                                });
                                modifications.push(gain_ab_mod);
                                modifications
                            },
                            span: reference.span().merge(&ability.span()),
                        },
                    ),
                    duration: crate::ability_tree::time::ForwardDuration::ObjectLifetime {
                        span: ability.span().empty_at_end(),
                    },
                    span: reference.span().merge(&ability.span()),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }];

    /* Objects can "gain" abilities, but it means it is for some duration. */
    let object_gains_abilities_until = [
        crate::ability_tree::time::ForwardDuration::UntilEndOfTurn {
            span: Default::default(),
        },
        crate::ability_tree::time::ForwardDuration::UntilEndOfYourNextTurn {
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|duration| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                span: Default::default(),
            }))
            .id(),
            ParserNode::AbilityKind { ability: dummy() }.id(),
            ParserNode::LexerToken(Token::ForwardDuration(duration)).id(),
        ]),
        merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain { span })),
                ParserNode::AbilityKind { ability },
                ParserNode::LexerToken(Token::ForwardDuration(duration)),
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
                                    span: span.merge(&ability.span()),
                                });
                                modifications.push(gain_ab_mod);
                                modifications
                            },
                            span: reference.span().merge(&ability.span()),
                        },
                    ),
                    duration: duration.clone(),
                    span: reference.span().merge(&duration.span()),
                },
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
