use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::ability::statik::continuous_effect;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* Objects can't block */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference {
                    reference: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Cant {
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
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Cant { .. })),
                    ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Blocks {
                        #[cfg(feature = "spanned_tree")]
                            span: block_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::EventCantHappen(
                            continuous_effect::ContinuousEventCantHappen {
                                event: crate::ability_tree::event::Event::CreatureAction(
                                    crate::ability_tree::event::CreatureActionEvent {
                                        creatures: reference.clone(),
                                        action: crate::ability_tree::event::CreatureAction::Blocks(
                                            crate::ability_tree::event::CreatureBlocksAction {
                                                blocked_creature: None,
                                                #[cfg(feature = "spanned_tree")]
                                                span: *block_span,
                                            },
                                        ),
                                        #[cfg(feature = "spanned_tree")]
                                        span: *block_span,
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: *block_span,
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: reference.span().merge(block_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Objects can't block specific objects */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference {
                    reference: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Cant {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Blocks {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference {
                    reference: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference: blockers },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Cant { .. })),
                    ParserNode::LexerToken(Token::CardActions(intermediate::CardActions::Blocks {
                        #[cfg(feature = "spanned_tree")]
                            span: block_span,
                    })),
                    ParserNode::ObjectReference { reference: blockee },
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::EventCantHappen(
                            continuous_effect::ContinuousEventCantHappen {
                                event: crate::ability_tree::event::Event::CreatureAction(
                                    crate::ability_tree::event::CreatureActionEvent {
                                        creatures: blockers.clone(),
                                        action: crate::ability_tree::event::CreatureAction::Blocks(
                                            crate::ability_tree::event::CreatureBlocksAction {
                                                blocked_creature: Some(blockee.clone()),
                                                #[cfg(feature = "spanned_tree")]
                                                span: *block_span,
                                            },
                                        ),
                                        #[cfg(feature = "spanned_tree")]
                                        span: *block_span,
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: *block_span,
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: blockers.span().merge(block_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
