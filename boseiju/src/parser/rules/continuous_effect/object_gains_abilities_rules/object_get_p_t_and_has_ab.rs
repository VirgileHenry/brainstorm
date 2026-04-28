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
        /* "<object> gets +n/+n and has <ability>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Creature { creature: dummy() }.id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::PowerToughnessModifiers { modifiers: dummy() }.id(),
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
                ParserNode::Ability { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Creature { creature },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                    ParserNode::PowerToughnessModifiers { modifiers },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                        #[cfg(feature = "spanned_tree")]
                            span: gain_ab_span,
                    })),
                    ParserNode::Ability { ability },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                            object: creature.to_permanent(),
                            modifications: {
                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                    ObjectCharacteristicModification::PowerToughnessModifiers(modifiers.clone()),
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
                            span: creature.node_span().merge(&ability.node_span()),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span().merge(&ability.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object> gets +n/+n and has <ability> and <ability> */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Creature { creature: dummy() }.id(),
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::PowerToughnessModifiers { modifiers: dummy() }.id(),
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
                ParserNode::Ability { ability: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Ability { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Creature { creature },
                    ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                    ParserNode::PowerToughnessModifiers { modifiers },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                        #[cfg(feature = "spanned_tree")]
                            span: ab1_span,
                    })),
                    ParserNode::Ability { ability: ability_1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::Ability { ability: ability_2 },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                            object: creature.to_permanent(),
                            modifications: {
                                let mut modifications = crate::utils::HeapArrayVec::new();
                                let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                    ObjectCharacteristicModification::PowerToughnessModifiers(modifiers.clone()),
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
                            span: creature.node_span().merge(&ability_2.node_span()),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span().merge(&ability_2.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
