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
                                    ability: crate::ability_tree::ability::Ability::KeywordAbility(keyword_ability.clone()),
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
        /* "<object> have "<ability>"" is a continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::QuotationMark {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Ability { ability: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::QuotationMark {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
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
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::QuotationMark { .. })),
                    ParserNode::Ability { ability },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::QuotationMark {
                        #[cfg(feature = "spanned_tree")]
                            span: ab_end_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                            object: permanent.clone(),
                            modifications: {
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
                            span: permanent.node_span().merge(&ability.node_span()),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.node_span().merge(ab_end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
