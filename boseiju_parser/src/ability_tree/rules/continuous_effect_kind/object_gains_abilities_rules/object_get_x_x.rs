use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_lexer::intermediate::NumberOperation;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::ContinuousEffect;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::continuous_effect_kind;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<object> gets +X/+0, where <x definition>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Creature {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                    token: intermediate::ActionKeyword::Get {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::XDefinition {
                    definition: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Creature { creature },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                        token: intermediate::ActionKeyword::Get { .. },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: x_span,
                    })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol { .. })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol { .. })),
                    ParserNode::Number { number: toughness },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::XDefinition { definition },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: continuous_effect_kind::ContinuousEffectKind::ModifyObjectAbilities(
                            continuous_effect_kind::ModifyObjectEffect {
                                object: creature.to_permanent(),
                                modifications: {
                                    let mut modifications = boseiju_tree::HeapArrayVec::new();
                                    let characteristic_mod =
                                        continuous_effect_kind::ObjectAbilitiesModification::CharacteristicModification(
                                            continuous_effect_kind::ObjectCharacteristicModification::PowerToughnessModifiers(
                                                continuous_effect_kind::PowerToughnessModifiers::PlusPlus(
                                                    continuous_effect_kind::PowerToughnessModifiersPlusPlus {
                                                        power_mod: boseiju_tree::ability_tree::number::Number::X(
                                                            boseiju_tree::ability_tree::number::XNumber {
                                                                x_definition: Box::new(definition.clone()),
                                                                #[cfg(feature = "spanned_tree")]
                                                                span: definition.span().merge(x_span),
                                                            },
                                                        ),
                                                        toughness_mod: toughness.clone(),
                                                        #[cfg(feature = "spanned_tree")]
                                                        span: span.merge(&toughness.span()),
                                                    },
                                                ),
                                            ),
                                        );

                                    modifications.push(characteristic_mod);
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: creature.span().merge(&definition.span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(&definition.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object> gets +X/+X, where <x_definition>" -> continuous effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Creature {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                    token: intermediate::ActionKeyword::Get {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::XDefinition {
                    definition: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Creature { creature },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                        token: intermediate::ActionKeyword::Get { .. },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: power_x_span,
                    })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol { .. })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol { .. })),
                    ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: toughness_x_span,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::XDefinition { definition },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: continuous_effect_kind::ContinuousEffectKind::ModifyObjectAbilities(
                            continuous_effect_kind::ModifyObjectEffect {
                                object: creature.to_permanent(),
                                modifications: {
                                    let mut modifications = boseiju_tree::HeapArrayVec::new();
                                    let characteristic_mod =
                                        continuous_effect_kind::ObjectAbilitiesModification::CharacteristicModification(
                                            continuous_effect_kind::ObjectCharacteristicModification::PowerToughnessModifiers(
                                                continuous_effect_kind::PowerToughnessModifiers::PlusPlus(
                                                    continuous_effect_kind::PowerToughnessModifiersPlusPlus {
                                                        power_mod: boseiju_tree::ability_tree::number::Number::X(
                                                            boseiju_tree::ability_tree::number::XNumber {
                                                                x_definition: Box::new(definition.clone()),
                                                                #[cfg(feature = "spanned_tree")]
                                                                span: definition.span().merge(power_x_span),
                                                            },
                                                        ),
                                                        toughness_mod: boseiju_tree::ability_tree::number::Number::X(
                                                            boseiju_tree::ability_tree::number::XNumber {
                                                                x_definition: Box::new(definition.clone()),
                                                                #[cfg(feature = "spanned_tree")]
                                                                span: definition.span().merge(toughness_x_span),
                                                            },
                                                        ),
                                                        #[cfg(feature = "spanned_tree")]
                                                        span: span.merge(toughness_x_span),
                                                    },
                                                ),
                                            ),
                                        );

                                    modifications.push(characteristic_mod);
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: creature.span().merge(&definition.span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(&definition.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object> gets +X/+0 until end of turn", where <x definition>" is a generate continuous effect thingy */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Creature {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                    token: intermediate::ActionKeyword::Get {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ForwardDuration {
                    duration: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::XDefinition {
                    definition: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Creature { creature },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                        token: intermediate::ActionKeyword::Get { .. },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: power_x_span,
                    })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol { .. })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol { .. })),
                    ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: toughness_x_span,
                    })),
                    ParserNode::ForwardDuration { duration },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::XDefinition { definition },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                        boseiju_tree::ability_tree::imperative::GenerateContinuousEffectImperative {
                            effect: ContinuousEffect {
                                effect: continuous_effect_kind::ContinuousEffectKind::ModifyObjectAbilities(continuous_effect_kind::ModifyObjectEffect {
                                    object: creature.to_permanent(),
                                    modifications: {
                                        /* Fixme: what a mess */
                                        let mut modifications = boseiju_tree::HeapArrayVec::new();
                                        let characteristic_mod = continuous_effect_kind::ObjectAbilitiesModification::CharacteristicModification(
                                            continuous_effect_kind::ObjectCharacteristicModification::PowerToughnessModifiers(
                                                continuous_effect_kind::PowerToughnessModifiers::PlusPlus(continuous_effect_kind::PowerToughnessModifiersPlusPlus {
                                                    power_mod: boseiju_tree::ability_tree::number::Number::X(
                                                        boseiju_tree::ability_tree::number::XNumber {
                                                            x_definition: Box::new(definition.clone()),
                                                            #[cfg(feature = "spanned_tree")]
                                                            span: definition.span().merge(power_x_span),
                                                        },
                                                    ),
                                                    toughness_mod: boseiju_tree::ability_tree::number::Number::X(
                                                        boseiju_tree::ability_tree::number::XNumber {
                                                            x_definition: Box::new(definition.clone()),
                                                            #[cfg(feature = "spanned_tree")]
                                                            span: definition.span().merge(toughness_x_span),
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
                                    span: creature.span().merge(&definition.span()),
                                }),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.span().merge(&definition.span()),
                            },
                            duration: duration.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: creature.span().merge(&definition.span()),
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
                ParserNode::Creature {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                    token: intermediate::ActionKeyword::Get {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::ForwardDuration {
                    duration: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::XDefinition {
                    definition: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Creature { creature },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                        token: intermediate::ActionKeyword::Get { .. },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: x_span,
                    })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol { .. })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::PlusSymbol { .. })),
                    ParserNode::Number { number: toughness },
                    ParserNode::ForwardDuration { duration },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::XDefinition { definition },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                        boseiju_tree::ability_tree::imperative::GenerateContinuousEffectImperative {
                            effect: ContinuousEffect {
                                effect: continuous_effect_kind::ContinuousEffectKind::ModifyObjectAbilities(continuous_effect_kind::ModifyObjectEffect {
                                    object: creature.to_permanent(),
                                    modifications: {
                                        let mut modifications = boseiju_tree::HeapArrayVec::new();
                                        let characteristic_mod = continuous_effect_kind::ObjectAbilitiesModification::CharacteristicModification(
                                            continuous_effect_kind::ObjectCharacteristicModification::PowerToughnessModifiers(
                                                continuous_effect_kind::PowerToughnessModifiers::PlusPlus(continuous_effect_kind::PowerToughnessModifiersPlusPlus {
                                                    power_mod: boseiju_tree::ability_tree::number::Number::X(
                                                        boseiju_tree::ability_tree::number::XNumber {
                                                            x_definition: Box::new(definition.clone()),
                                                            #[cfg(feature = "spanned_tree")]
                                                            span: definition.span().merge(x_span),
                                                        },
                                                    ),
                                                    toughness_mod: toughness.clone(),
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: span.merge(&toughness.span()),
                                                }),
                                            ),
                                        );

                                        modifications.push(characteristic_mod);
                                        modifications
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: creature.span().merge(&definition.span()),
                                }),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.span().merge(&definition.span()),
                            },
                            duration: duration.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: creature.span().merge(&definition.span()),
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
