use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::ContinuousEffect;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::continuous_effect_kind;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<object> gets +n/+n and has <ability>" */
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
                ParserNode::PowerToughnessModifiers {
                    modifiers: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Have {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::Ability {
                    ability: Default::default(),
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
                    ParserNode::PowerToughnessModifiers { modifiers },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::And { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token:
                            intermediate::EnglishVerb::Have {
                                #[cfg(feature = "spanned_tree")]
                                    span: gain_ab_span,
                            },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::Ability { ability },
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
                                                modifiers.clone(),
                                            ),
                                        );
                                    let gain_ab_mod = continuous_effect_kind::ObjectAbilitiesModification::GainAbility(
                                        continuous_effect_kind::ObjectGainAbility {
                                            ability: boseiju_tree::AbilityTree::from_single_ability(ability.clone()),
                                            #[cfg(feature = "spanned_tree")]
                                            span: gain_ab_span.merge(&ability.span()),
                                        },
                                    );

                                    modifications.push(characteristic_mod);
                                    modifications.push(gain_ab_mod);
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: creature.span().merge(&ability.span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(&ability.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object> gets +n/+n and has <ability> and <ability> */
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
                ParserNode::PowerToughnessModifiers {
                    modifiers: Default::default(),
                }
                .id(),
                #[cfg(feature = "spanned_tree")]
                ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::And {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Have {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                }))
                .id(),
                ParserNode::Ability {
                    ability: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Ability {
                    ability: Default::default(),
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
                    ParserNode::PowerToughnessModifiers { modifiers },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::And { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token:
                            intermediate::EnglishVerb::Have {
                                #[cfg(feature = "spanned_tree")]
                                    span: ab1_span,
                            },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::Ability { ability: ability_1 },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::And { .. })),
                    ParserNode::Ability { ability: ability_2 },
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
                                                modifiers.clone(),
                                            ),
                                        );
                                    let gain_ab1_mod = continuous_effect_kind::ObjectAbilitiesModification::GainAbility(
                                        continuous_effect_kind::ObjectGainAbility {
                                            ability: boseiju_tree::AbilityTree::from_single_ability(ability_1.clone()),
                                            #[cfg(feature = "spanned_tree")]
                                            span: ab1_span.merge(&ability_1.span()),
                                        },
                                    );
                                    let gain_ab2_mod = continuous_effect_kind::ObjectAbilitiesModification::GainAbility(
                                        continuous_effect_kind::ObjectGainAbility {
                                            ability: boseiju_tree::AbilityTree::from_single_ability(ability_2.clone()),
                                            #[cfg(feature = "spanned_tree")]
                                            span: ability_2.span(),
                                        },
                                    );

                                    modifications.push(characteristic_mod);
                                    modifications.push(gain_ab1_mod);
                                    modifications.push(gain_ab2_mod);
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: creature.span().merge(&ability_2.span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(&ability_2.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
