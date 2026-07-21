use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::ContinuousEffect;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::continuous_effect_kind;
use boseiju_tree::ability_tree::action;
use boseiju_tree::ability_tree::object;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<creature> can't block" is a rule modification effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Creature {
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
                    ParserNode::Creature { creature },
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::Cant { .. })),
                    ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Blocks {
                        #[cfg(feature = "spanned_tree")]
                            span: block_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: continuous_effect_kind::ContinuousEffectKind::ModifyRule(
                            continuous_effect_kind::ModifyRuleEffect::CreatureCantDoAction(
                                continuous_effect_kind::CreatureCantDoAction {
                                    action: action::CreatureAction::Blocks(action::CreatureBlocksAction {
                                        creature: creature.clone(),
                                        blocked_creature: None,
                                        #[cfg(feature = "spanned_tree")]
                                        span: creature.span().merge(block_span),
                                    }),
                                    #[cfg(feature = "spanned_tree")]
                                    span: creature.span().merge(block_span),
                                },
                            ),
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
                ParserNode::Creature {
                    creature: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::Cant {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Be {
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
                    ParserNode::Creature { creature },
                    ParserNode::LexerToken(Token::EnglishModalAuxiliary(intermediate::EnglishModalAuxiliary::Cant { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Be { .. })),
                    ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Blocks {
                        #[cfg(feature = "spanned_tree")]
                            span: block_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: ContinuousEffect {
                        effect: continuous_effect_kind::ContinuousEffectKind::ModifyRule(
                            continuous_effect_kind::ModifyRuleEffect::CreatureCantDoAction(
                                continuous_effect_kind::CreatureCantDoAction {
                                    action: action::CreatureAction::Blocks(action::CreatureBlocksAction {
                                        creature: object::Creature::Reference(object::reference::CreatureReference {
                                            count: object::CountSpecifier::All {
                                                #[cfg(feature = "spanned_tree")]
                                                span: block_span.empty_at_end(),
                                            },
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
                                },
                            ),
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
