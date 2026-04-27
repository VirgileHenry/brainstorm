use crate::ability_tree::ability::statik::continuous_effect::*;
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

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<object> gets +X/+0, where <x definition>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureReference { creature: dummy() }.id(),
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
                ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
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
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::XDefinition { definition: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureReference { creature },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: x_span,
                    })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                    ParserNode::Number { number: toughness },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::XDefinition { definition },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                            object: crate::ability_tree::object::PermanentReference::Creature(creature.clone()),
                            modifications: {
                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                    ObjectCharacteristicModification::PowerToughnessModifiers(PowerToughnessModifiers::PlusPlus(
                                        PowerToughnessModifiersPlusPlus {
                                            power_mod: crate::ability_tree::number::Number::X(
                                                crate::ability_tree::number::XNumber {
                                                    x_definition: Box::new(definition.clone()),
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: definition.node_span().merge(x_span),
                                                },
                                            ),
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
                            span: creature.node_span().merge(&definition.node_span()),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span().merge(&definition.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object> gets +X/+X, where <x_definition>" -> continuous effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureReference { creature: dummy() }.id(),
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
                ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
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
                ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::XDefinition { definition: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureReference { creature },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: power_x_span,
                    })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: toughness_x_span,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::XDefinition { definition },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                            object: crate::ability_tree::object::PermanentReference::Creature(creature.clone()),
                            modifications: {
                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                    ObjectCharacteristicModification::PowerToughnessModifiers(PowerToughnessModifiers::PlusPlus(
                                        PowerToughnessModifiersPlusPlus {
                                            power_mod: crate::ability_tree::number::Number::X(
                                                crate::ability_tree::number::XNumber {
                                                    x_definition: Box::new(definition.clone()),
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: definition.node_span().merge(power_x_span),
                                                },
                                            ),
                                            toughness_mod: crate::ability_tree::number::Number::X(
                                                crate::ability_tree::number::XNumber {
                                                    x_definition: Box::new(definition.clone()),
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: definition.node_span().merge(toughness_x_span),
                                                },
                                            ),
                                            #[cfg(feature = "spanned_tree")]
                                            span: span.merge(toughness_x_span),
                                        },
                                    )),
                                );

                                modifications.push(characteristic_mod);
                                modifications
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: creature.node_span().merge(&definition.node_span()),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span().merge(&definition.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object> gets +X/+0 until end of turn", where <x definition>" is a generate continuous effect thingy */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureReference { creature: dummy() }.id(),
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
                ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
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
                ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ForwardDuration { duration: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::XDefinition { definition: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureReference { creature },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: power_x_span,
                    })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: toughness_x_span,
                    })),
                    ParserNode::ForwardDuration { duration },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::XDefinition { definition },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                        crate::ability_tree::imperative::GenerateContinuousEffectImperative {
                            effect: ContinuousEffect {
                                effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                                    object: crate::ability_tree::object::PermanentReference::Creature(creature.clone()),
                                    modifications: {
                                        let mut modifications = crate::utils::HeapArrayVec::new();
                                        let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                            ObjectCharacteristicModification::PowerToughnessModifiers(
                                                PowerToughnessModifiers::PlusPlus(PowerToughnessModifiersPlusPlus {
                                                    power_mod: crate::ability_tree::number::Number::X(
                                                        crate::ability_tree::number::XNumber {
                                                            x_definition: Box::new(definition.clone()),
                                                            #[cfg(feature = "spanned_tree")]
                                                            span: definition.node_span().merge(power_x_span),
                                                        },
                                                    ),
                                                    toughness_mod: crate::ability_tree::number::Number::X(
                                                        crate::ability_tree::number::XNumber {
                                                            x_definition: Box::new(definition.clone()),
                                                            #[cfg(feature = "spanned_tree")]
                                                            span: definition.node_span().merge(toughness_x_span),
                                                        },
                                                    ),
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: span.merge(&toughness_x_span),
                                                }),
                                            ),
                                        );

                                        modifications.push(characteristic_mod);
                                        modifications
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: creature.node_span().merge(&definition.node_span()),
                                }),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.node_span().merge(&definition.node_span()),
                            },
                            duration: duration.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: creature.node_span().merge(&definition.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object> gets +X/+X until end of turn", where <x definition>" is a generate continuous effect thingy */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureReference { creature: dummy() }.id(),
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
                ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
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
                ParserNode::ForwardDuration { duration: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::XDefinition { definition: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureReference { creature },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: x_span,
                    })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                    ParserNode::Number { number: toughness },
                    ParserNode::ForwardDuration { duration },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::XDefinition { definition },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                        crate::ability_tree::imperative::GenerateContinuousEffectImperative {
                            effect: ContinuousEffect {
                                effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                                    object: crate::ability_tree::object::PermanentReference::Creature(creature.clone()),
                                    modifications: {
                                        let mut modifications = crate::utils::HeapArrayVec::new();
                                        let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                            ObjectCharacteristicModification::PowerToughnessModifiers(
                                                PowerToughnessModifiers::PlusPlus(PowerToughnessModifiersPlusPlus {
                                                    power_mod: crate::ability_tree::number::Number::X(
                                                        crate::ability_tree::number::XNumber {
                                                            x_definition: Box::new(definition.clone()),
                                                            #[cfg(feature = "spanned_tree")]
                                                            span: definition.node_span().merge(x_span),
                                                        },
                                                    ),
                                                    toughness_mod: toughness.clone(),
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: span.merge(&toughness.node_span()),
                                                }),
                                            ),
                                        );

                                        modifications.push(characteristic_mod);
                                        modifications
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: creature.node_span().merge(&definition.node_span()),
                                }),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.node_span().merge(&definition.node_span()),
                            },
                            duration: duration.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: creature.node_span().merge(&definition.node_span()),
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
