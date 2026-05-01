use crate::ability_tree::ability::statik::continuous_effect::*;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
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
        /* "<permanent reference> have <keyword ability>" is a continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::KeywordAbility { keyword_ability },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                            object: permanent.clone(),
                            modifications: {
                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let gain_ab_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: crate::AbilityTree::from_single_ability(
                                        crate::ability_tree::ability::Ability::KeywordAbility(keyword_ability.clone()),
                                    ),
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.merge(&keyword_ability.node_span()),
                                });
                                modifications.push(gain_ab_mod);
                                modifications
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.node_span().merge(&keyword_ability.node_span()),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.node_span().merge(&keyword_ability.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent reference> have <keyword ability> and <keyword ability>" is a continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have { .. })),
                    ParserNode::KeywordAbility {
                        keyword_ability: kw_ab_1,
                    },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::KeywordAbility {
                        keyword_ability: kw_ab_2,
                    },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                            object: permanent.clone(),
                            modifications: {
                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let gain_ab_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: crate::AbilityTree {
                                        abilities: [
                                            crate::ability_tree::ability::Ability::KeywordAbility(kw_ab_1.clone()),
                                            crate::ability_tree::ability::Ability::KeywordAbility(kw_ab_2.clone()),
                                        ]
                                        .into_iter()
                                        .collect(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: permanent.node_span().merge(&kw_ab_2.node_span()),
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: permanent.node_span().merge(&kw_ab_2.node_span()),
                                });
                                modifications.push(gain_ab_mod);
                                modifications
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.node_span().merge(&kw_ab_2.node_span()),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.node_span().merge(&kw_ab_2.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent reference> have <keyword ability>, <keyword ability>, and <keyword ability>" is a continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have { .. })),
                    ParserNode::KeywordAbility {
                        keyword_ability: kw_ab_1,
                    },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::KeywordAbility {
                        keyword_ability: kw_ab_2,
                    },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::KeywordAbility {
                        keyword_ability: kw_ab_3,
                    },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                            object: permanent.clone(),
                            modifications: {
                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let gain_ab_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: crate::AbilityTree {
                                        abilities: [
                                            crate::ability_tree::ability::Ability::KeywordAbility(kw_ab_1.clone()),
                                            crate::ability_tree::ability::Ability::KeywordAbility(kw_ab_2.clone()),
                                            crate::ability_tree::ability::Ability::KeywordAbility(kw_ab_3.clone()),
                                        ]
                                        .into_iter()
                                        .collect(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: permanent.node_span().merge(&kw_ab_3.node_span()),
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: permanent.node_span().merge(&kw_ab_3.node_span()),
                                });
                                modifications.push(gain_ab_mod);
                                modifications
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.node_span().merge(&kw_ab_3.node_span()),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.node_span().merge(&kw_ab_3.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object> have "<ability>"" is a continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Ability { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::Ability { ability },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                            object: permanent.clone(),
                            modifications: {
                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let gain_ab_mod = ObjectAbilitiesModification::GainAbility(ObjectGainAbility {
                                    ability: crate::AbilityTree::from_single_ability(ability.clone()),
                                    #[cfg(feature = "spanned_tree")]
                                    span: span.merge(&ability.node_span()),
                                });
                                modifications.push(gain_ab_mod);
                                modifications
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.node_span().merge(&ability.node_span()),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.node_span().merge(&ability.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
