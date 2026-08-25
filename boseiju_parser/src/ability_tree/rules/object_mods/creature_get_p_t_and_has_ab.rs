use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::object_mods::object_mod;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<object> gets +n/+n and has <ability>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureActive {
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
            merged: ParserNode::ObjectModsEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureActive { creature },
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
                                    span: have_ab_span,
                            },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::Ability { ability },
                ] => Ok(ParserNode::ObjectModsEffect {
                    effect: boseiju_tree::ability_tree::object_mods::ObjectModsEffect {
                        object: creature.to_permanent(),
                        modifications: [
                            object_mod::ObjectMod::CreaturePowerToughness(
                                object_mod::creature_power_toughness_modifier::CreaturePowerToughnessModifier {
                                    modifier: modifiers.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: modifiers.span(),
                                },
                            ),
                            object_mod::ObjectMod::GainAbility(object_mod::ObjectGainAbility {
                                ability: boseiju_tree::AbilityTree::from_single_ability(ability.clone()),
                                #[cfg(feature = "spanned_tree")]
                                span: ability.span().merge(have_ab_span),
                            }),
                        ]
                        .into_iter()
                        .collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(&modifiers.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object> gets +n/+n and gain <ability>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureActive {
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
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Ability {
                    ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ObjectModsEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreatureActive { creature },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediate::TensedActionKeyword {
                        token: intermediate::ActionKeyword::Get { .. },
                        tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                    })),
                    ParserNode::PowerToughnessModifiers { modifiers },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::And { .. })),
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Gain {
                        #[cfg(feature = "spanned_tree")]
                            span: gain_ab_span,
                    })),
                    ParserNode::Ability { ability },
                ] => Ok(ParserNode::ObjectModsEffect {
                    effect: boseiju_tree::ability_tree::object_mods::ObjectModsEffect {
                        object: creature.to_permanent(),
                        modifications: [
                            object_mod::ObjectMod::CreaturePowerToughness(
                                object_mod::creature_power_toughness_modifier::CreaturePowerToughnessModifier {
                                    modifier: modifiers.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: modifiers.span(),
                                },
                            ),
                            object_mod::ObjectMod::GainAbility(object_mod::ObjectGainAbility {
                                ability: boseiju_tree::AbilityTree::from_single_ability(ability.clone()),
                                #[cfg(feature = "spanned_tree")]
                                span: ability.span().merge(gain_ab_span),
                            }),
                        ]
                        .into_iter()
                        .collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(&modifiers.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
