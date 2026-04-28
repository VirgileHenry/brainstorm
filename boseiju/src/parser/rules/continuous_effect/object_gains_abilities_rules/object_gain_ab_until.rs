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
        /* "<permanent reference> gain <ability> <forward duration>" is a generate continuous effect imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Ability { ability: dummy() }.id(),
                ParserNode::ForwardDuration { duration: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::Ability { ability },
                    ParserNode::ForwardDuration { duration },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                        crate::ability_tree::imperative::GenerateContinuousEffectImperative {
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
                                span: permanent.node_span().merge(&duration.node_span()),
                            },
                            duration: duration.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.node_span().merge(&duration.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object> gain <ability>" is forever. It usually happens when the object also get sacrificed at some point */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Permanent { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Ability { ability: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Gain {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::Ability { ability },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                        crate::ability_tree::imperative::GenerateContinuousEffectImperative {
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
                                span: permanent.node_span().merge(&ability.node_span()),
                            },
                            duration: crate::ability_tree::time::ForwardDuration::Forever {
                                #[cfg(feature = "spanned_tree")]
                                span: ability.node_span().empty_at_end(),
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.node_span().merge(&ability.node_span()),
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
