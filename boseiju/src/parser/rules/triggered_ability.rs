use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Whenever, an event, a comma and a statement make the structure for triggered abilities. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Whenever {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::TriggerCondition { condition: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Whenever {
                        #[cfg(feature = "spanned_tree")]
                            span: whenever_span,
                    })),
                    ParserNode::TriggerCondition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::Triggered(
                        crate::ability_tree::ability::triggered::TriggeredAbility {
                            condition: condition.clone(),
                            effect: ability.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: whenever_span.merge(&ability.span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* When, an event, a comma and a statement also make a structure for triggered abilities. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::When {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::TriggerCondition { condition: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::When {
                        #[cfg(feature = "spanned_tree")]
                            span: when_span,
                    })),
                    ParserNode::TriggerCondition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::Triggered(
                        crate::ability_tree::ability::triggered::TriggeredAbility {
                            condition: condition.clone(),
                            effect: ability.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: when_span.merge(&ability.span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Event on their own can make trigger conditions */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Event { event: dummy() }.id()]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Event { event }] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        event: event.clone(),
                        condition: None,
                        #[cfg(feature = "spanned_tree")]
                        span: event.node_span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Event with a comma and a if condition can also make trigger conditions */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Event { event: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition { condition: dummy() }.id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Event { event },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                        #[cfg(feature = "spanned_tree")]
                            span: if_span,
                    })),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        event: event.clone(),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: if_span.merge(&condition.node_span()),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: event.node_span().merge(&condition.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Special case for the "during your turn" if condition that may not appear behind a comma */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Event { event: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::During {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                /* Fixme: a bit weird for a "your turn" ? Maybe it shall be a single token */
                ParserNode::LexerToken(Token::OwnerSpecifier(terminals::OwnerSpecifier::YouOwn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Turn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Event { event },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::During {
                        #[cfg(feature = "spanned_tree")]
                            span: during_span,
                    })),
                    /* Fixme: a bit weird for a "your turn" ? Maybe it shall be a single token */
                    ParserNode::LexerToken(Token::OwnerSpecifier(terminals::OwnerSpecifier::YouOwn { .. })),
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Turn {
                        #[cfg(feature = "spanned_tree")]
                            span: turn_span,
                    })),
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        event: event.clone(),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: crate::ability_tree::conditional::Condition::ThisIsYourTurn(
                                    crate::ability_tree::conditional::ConditionThisIsYourTurn {
                                        #[cfg(feature = "spanned_tree")]
                                        span: during_span.merge(turn_span),
                                    },
                                ),
                                #[cfg(feature = "spanned_tree")]
                                span: during_span.merge(turn_span),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: event.node_span().merge(turn_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
