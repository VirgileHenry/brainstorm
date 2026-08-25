use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::action;
use boseiju_tree::ability_tree::continuous_effect;
use boseiju_tree::ability_tree::object;
use boseiju_tree::ability_tree::quantifier;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<creature> can't block" is a rule modification effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreaturePassive {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::Cant {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Blocks {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreaturePassive { creature },
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::Cant { .. })),
                    ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Blocks {
                        #[cfg(feature = "spanned_tree")]
                            span: block_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::ModifyRule(
                            continuous_effect::ModifyRuleEffect::CreatureCantDoAction(continuous_effect::CreatureCantDoAction {
                                action: action::CreatureAction::Blocks(action::CreatureBlocksAction {
                                    creature: creature.clone(),
                                    blocked_creature: None,
                                    #[cfg(feature = "spanned_tree")]
                                    span: creature.span().merge(block_span),
                                }),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.span().merge(block_span),
                            }),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(block_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<creature> can't be blocked" is a rule modification effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreaturePassive {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::Cant {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Be {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Blocks {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CreaturePassive { creature },
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::Cant { .. })),
                    ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                        token: intermediate::EnglishVerb::Be { .. },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Blocks {
                        #[cfg(feature = "spanned_tree")]
                            span: block_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::ModifyRule(
                            continuous_effect::ModifyRuleEffect::CreatureCantDoAction(continuous_effect::CreatureCantDoAction {
                                action: action::CreatureAction::Blocks(action::CreatureBlocksAction {
                                    creature: object::Creature::Reference(object::reference::CreatureReference {
                                        quantifier: quantifier::PassiveQuantifier::All(quantifier::All {
                                            #[cfg(feature = "spanned_tree")]
                                            span: block_span.empty_at_end(),
                                        }),
                                        creature: object::specified_object::SpecifiedCreature {
                                            kind: object::kind::CreatureKind::Creature {
                                                #[cfg(feature = "spanned_tree")]
                                                span: block_span.empty_at_end(),
                                            },
                                            specifiers: None,
                                            #[cfg(feature = "spanned_tree")]
                                            span: block_span.empty_at_end(),
                                        },
                                        #[cfg(feature = "spanned_tree")]
                                        span: block_span.empty_at_end(),
                                    }),
                                    blocked_creature: Some(creature.clone()),
                                    #[cfg(feature = "spanned_tree")]
                                    span: creature.span().merge(block_span),
                                }),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.span().merge(block_span),
                            }),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.span().merge(block_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
