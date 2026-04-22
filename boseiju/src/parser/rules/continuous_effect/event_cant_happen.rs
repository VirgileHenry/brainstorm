use crate::ability_tree::ability::statik::continuous_effect;
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
        /* Objects can't block */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Cant {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Blocks {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Cant { .. })),
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Blocks {
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
                        span: reference.node_span().merge(block_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Objects can't block specific objects */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Cant {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Blocks {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference: blockers },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Cant { .. })),
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Blocks {
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
                        span: blockers.node_span().merge(block_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
