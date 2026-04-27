use crate::ability_tree::ability::statik::continuous_effect;
use crate::ability_tree::action;
use crate::ability_tree::object;
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
        /* "<creature reference> can't block" is a rule modification effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureReference { creature: dummy() }.id(),
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
                    ParserNode::CreatureReference { creature },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Cant { .. })),
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Blocks {
                        #[cfg(feature = "spanned_tree")]
                            span: block_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::ModifyRule(
                            continuous_effect::ModifyRuleEffect::CreatureCantDoAction(continuous_effect::CreatureCantDoAction {
                                action: action::CreatureAction::Blocks(action::CreatureBlocksAction {
                                    creature: creature.clone(),
                                    blocked_creature: None,
                                    #[cfg(feature = "spanned_tree")]
                                    span: creature.node_span().merge(block_span),
                                }),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.node_span().merge(block_span),
                            }),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span().merge(block_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<creature reference> can't be blocked" is a rule modification effect */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CreatureReference { creature: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Cant {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Be {
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
                    ParserNode::CreatureReference { creature },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Cant { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Be { .. })),
                    ParserNode::LexerToken(Token::CardActions(intermediates::CardActions::Blocks {
                        #[cfg(feature = "spanned_tree")]
                            span: block_span,
                    })),
                ] => Ok(ParserNode::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::ModifyRule(
                            continuous_effect::ModifyRuleEffect::CreatureCantDoAction(continuous_effect::CreatureCantDoAction {
                                action: action::CreatureAction::Blocks(action::CreatureBlocksAction {
                                    creature: object::CreatureReference::Specified(object::specified_object::SpecifiedCreature {
                                        amount: object::CountSpecifier::All {
                                            #[cfg(feature = "spanned_tree")]
                                            span: block_span.empty_at_end(),
                                        },
                                        specifiers: None,
                                        #[cfg(feature = "spanned_tree")]
                                        span: block_span.empty_at_end(),
                                    }),
                                    blocked_creature: Some(creature.clone()),
                                    #[cfg(feature = "spanned_tree")]
                                    span: creature.node_span().merge(block_span),
                                }),
                                #[cfg(feature = "spanned_tree")]
                                span: creature.node_span().merge(block_span),
                            }),
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span().merge(block_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
