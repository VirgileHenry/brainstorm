use super::ParserNode;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::conditional;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<trigger_cond>, <spell ability>" make the structure for triggered abilities. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::TriggerCondition {
                    condition: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility {
                    ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::WrittenAbility {
                ability: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::TriggerCondition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::WrittenAbility {
                    ability: boseiju_tree::ability_tree::ability::WrittenAbility::Triggered(
                        boseiju_tree::ability_tree::ability::triggered::TriggeredAbility {
                            trigger_condition: condition.clone(),
                            effect: ability.clone(),
                            condition: None,
                            #[cfg(feature = "spanned_tree")]
                            span: condition.span.merge(&ability.span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<trigger_cond>, <spell ability> if <condition>" make the structure for triggered abilities. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::TriggerCondition {
                    condition: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility {
                    ability: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition {
                    condition: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::WrittenAbility {
                ability: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::TriggerCondition { condition: trigger_cond },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::SpellAbility { ability },
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                        #[cfg(feature = "spanned_tree")]
                            span: if_span,
                    })),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::WrittenAbility {
                    ability: boseiju_tree::ability_tree::ability::WrittenAbility::Triggered(
                        boseiju_tree::ability_tree::ability::triggered::TriggeredAbility {
                            trigger_condition: trigger_cond.clone(),
                            effect: ability.clone(),
                            condition: Some(conditional::Conditional::If(conditional::ConditionalIf {
                                condition: condition.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: condition.span().merge(if_span),
                            })),
                            #[cfg(feature = "spanned_tree")]
                            span: trigger_cond.span.merge(&ability.span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
