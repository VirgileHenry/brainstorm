use crate::ability_tree::ability::statik::continuous_effect;
use crate::ability_tree::time;
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
        /* "Until end of turn, <continuous effect>" makes a generated continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::ForwardDuration(time::ForwardDuration::UntilEndOfTurn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ContinuousEffect { effect: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ForwardDuration(time::ForwardDuration::UntilEndOfTurn {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::ContinuousEffect { effect },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                        crate::ability_tree::imperative::GenerateContinuousEffectImperative {
                            effect: effect.clone(),
                            duration: time::ForwardDuration::UntilEndOfTurn {
                                #[cfg(feature = "spanned_tree")]
                                span: *start_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: effect.node_span().merge(start_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<continuous effect> until end of turn" makes a generated continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ContinuousEffect { effect: dummy() }.id(),
                ParserNode::LexerToken(Token::ForwardDuration(time::ForwardDuration::UntilEndOfTurn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ContinuousEffect { effect },
                    ParserNode::LexerToken(Token::ForwardDuration(time::ForwardDuration::UntilEndOfTurn {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                        crate::ability_tree::imperative::GenerateContinuousEffectImperative {
                            effect: effect.clone(),
                            duration: time::ForwardDuration::UntilEndOfTurn {
                                #[cfg(feature = "spanned_tree")]
                                span: *end_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: effect.node_span().merge(end_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object reference> this turn has <ability>" makes a generated continuous effect. */
        /* Fixme: this only appears with: "the next <spell specifier> this turn has...", maybe we could be more restrictive */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PermanentReference { permanent: dummy() }.id(),
                ParserNode::LexerToken(Token::BackwardDuration(time::BackwardDuration::ThisTurn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
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
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PermanentReference { permanent },
                    ParserNode::LexerToken(Token::BackwardDuration(time::BackwardDuration::ThisTurn {
                        #[cfg(feature = "spanned_tree")]
                            span: this_turn_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Have {
                        #[cfg(feature = "spanned_tree")]
                            span: have_span,
                    })),
                    ParserNode::KeywordAbility { keyword_ability },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::GenerateContinuousEffect(
                        crate::ability_tree::imperative::GenerateContinuousEffectImperative {
                            effect: continuous_effect::ContinuousEffect {
                                effect: continuous_effect::ContinuousEffectKind::ModifyObjectAbilities(
                                    continuous_effect::ModifyObjectEffect {
                                        object: permanent.clone(),
                                        modifications: {
                                            let mut modifications = crate::utils::HeapArrayVec::new();
                                            let gain_ab_mod = continuous_effect::ObjectAbilitiesModification::GainAbility(
                                                continuous_effect::ObjectGainAbility {
                                                    ability: crate::ability_tree::ability::Ability::KeywordAbility(
                                                        keyword_ability.clone(),
                                                    ),
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: keyword_ability.node_span().merge(have_span),
                                                },
                                            );
                                            modifications.push(gain_ab_mod);
                                            modifications
                                        },
                                        #[cfg(feature = "spanned_tree")]
                                        span: permanent.node_span().merge(&keyword_ability.node_span()),
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.node_span().merge(&keyword_ability.node_span()),
                            },
                            duration: time::ForwardDuration::UntilEndOfTurn {
                                #[cfg(feature = "spanned_tree")]
                                span: *this_turn_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.node_span().merge(&keyword_ability.node_span()),
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
