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
        /* "<permanent reference> have <keyword ability>" is a continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Has {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Has {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::KeywordAbility { keyword_ability },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: continuous_effect_kind::ContinuousEffectKind::ModifyObjectAbilities(
                            continuous_effect_kind::ModifyObjectEffect {
                                object: permanent.clone(),
                                modifications: {
                                    let mut modifications = boseiju_tree::HeapArrayVec::new();
                                    let gain_ab_mod = continuous_effect_kind::ObjectAbilitiesModification::GainAbility(
                                        continuous_effect_kind::ObjectGainAbility {
                                            ability: boseiju_tree::AbilityTree::from_single_ability(
                                                boseiju_tree::ability_tree::ability::Ability::KeywordAbility(
                                                    keyword_ability.clone(),
                                                ),
                                            ),
                                            #[cfg(feature = "spanned_tree")]
                                            span: span.merge(&keyword_ability.span()),
                                        },
                                    );
                                    modifications.push(gain_ab_mod);
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().merge(&keyword_ability.span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(&keyword_ability.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent reference> have <keyword ability> and <keyword ability>" is a continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Has {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Has { .. })),
                    ParserNode::KeywordAbility {
                        keyword_ability: kw_ab_1,
                    },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::And { .. })),
                    ParserNode::KeywordAbility {
                        keyword_ability: kw_ab_2,
                    },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: continuous_effect_kind::ContinuousEffectKind::ModifyObjectAbilities(
                            continuous_effect_kind::ModifyObjectEffect {
                                object: permanent.clone(),
                                modifications: {
                                    let mut modifications = boseiju_tree::HeapArrayVec::new();
                                    let gain_ab_mod = continuous_effect_kind::ObjectAbilitiesModification::GainAbility(
                                        continuous_effect_kind::ObjectGainAbility {
                                            ability: boseiju_tree::AbilityTree {
                                                abilities: [
                                                    boseiju_tree::ability_tree::ability::Ability::KeywordAbility(kw_ab_1.clone()),
                                                    boseiju_tree::ability_tree::ability::Ability::KeywordAbility(kw_ab_2.clone()),
                                                ]
                                                .into_iter()
                                                .collect(),
                                                #[cfg(feature = "spanned_tree")]
                                                span: permanent.span().merge(&kw_ab_2.span()),
                                            },
                                            #[cfg(feature = "spanned_tree")]
                                            span: permanent.span().merge(&kw_ab_2.span()),
                                        },
                                    );
                                    modifications.push(gain_ab_mod);
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().merge(&kw_ab_2.span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(&kw_ab_2.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent reference> have <keyword ability>, <keyword ability>, and <keyword ability>" is a continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Has {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Has { .. })),
                    ParserNode::KeywordAbility {
                        keyword_ability: kw_ab_1,
                    },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::KeywordAbility {
                        keyword_ability: kw_ab_2,
                    },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::And { .. })),
                    ParserNode::KeywordAbility {
                        keyword_ability: kw_ab_3,
                    },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: continuous_effect_kind::ContinuousEffectKind::ModifyObjectAbilities(
                            continuous_effect_kind::ModifyObjectEffect {
                                object: permanent.clone(),
                                modifications: {
                                    let mut modifications = boseiju_tree::HeapArrayVec::new();
                                    let gain_ab_mod = continuous_effect_kind::ObjectAbilitiesModification::GainAbility(
                                        continuous_effect_kind::ObjectGainAbility {
                                            ability: boseiju_tree::AbilityTree {
                                                abilities: [
                                                    boseiju_tree::ability_tree::ability::Ability::KeywordAbility(kw_ab_1.clone()),
                                                    boseiju_tree::ability_tree::ability::Ability::KeywordAbility(kw_ab_2.clone()),
                                                    boseiju_tree::ability_tree::ability::Ability::KeywordAbility(kw_ab_3.clone()),
                                                ]
                                                .into_iter()
                                                .collect(),
                                                #[cfg(feature = "spanned_tree")]
                                                span: permanent.span().merge(&kw_ab_3.span()),
                                            },
                                            #[cfg(feature = "spanned_tree")]
                                            span: permanent.span().merge(&kw_ab_3.span()),
                                        },
                                    );
                                    modifications.push(gain_ab_mod);
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().merge(&kw_ab_3.span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(&kw_ab_3.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object> have "<ability>"" is a continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Has {
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
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Has {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::Ability { ability },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: continuous_effect_kind::ContinuousEffectKind::ModifyObjectAbilities(
                            continuous_effect_kind::ModifyObjectEffect {
                                object: permanent.clone(),
                                modifications: {
                                    let mut modifications = boseiju_tree::HeapArrayVec::new();
                                    let gain_ab_mod = continuous_effect_kind::ObjectAbilitiesModification::GainAbility(
                                        continuous_effect_kind::ObjectGainAbility {
                                            ability: boseiju_tree::AbilityTree::from_single_ability(ability.clone()),
                                            #[cfg(feature = "spanned_tree")]
                                            span: span.merge(&ability.span()),
                                        },
                                    );
                                    modifications.push(gain_ab_mod);
                                    modifications
                                },
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().merge(&ability.span()),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span().merge(&ability.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
