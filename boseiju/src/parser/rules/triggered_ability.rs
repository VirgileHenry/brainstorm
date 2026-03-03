use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Whenever, an event, a comma and a statement make the structure for triggered abilities. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Whenever {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::TriggerCondition { condition: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Whenever {
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
                    span: Default::default(),
                }))
                .id(),
                ParserNode::TriggerCondition { condition: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::When { span: when_span })),
                    ParserNode::TriggerCondition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::Triggered(
                        crate::ability_tree::ability::triggered::TriggeredAbility {
                            condition: condition.clone(),
                            effect: ability.clone(),
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
                        span: event.span(),
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
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
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
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If { span: if_span })),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        event: event.clone(),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                                span: if_span.merge(&condition.span()),
                            },
                        )),
                        span: event.span().merge(&condition.span()),
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
                    span: Default::default(),
                }))
                .id(),
                /* Fixme: a bit weird for a "your turn" ? Maybe it shall be a single token */
                ParserNode::LexerToken(Token::OwnerSpecifier(terminals::OwnerSpecifier::YouOwn {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Turn {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::TriggerCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Event { event },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::During { span: during_span })),
                    /* Fixme: a bit weird for a "your turn" ? Maybe it shall be a single token */
                    ParserNode::LexerToken(Token::OwnerSpecifier(terminals::OwnerSpecifier::YouOwn { .. })),
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Turn { span: turn_span })),
                ] => Ok(ParserNode::TriggerCondition {
                    condition: crate::ability_tree::ability::triggered::TriggerCondition {
                        event: event.clone(),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: crate::ability_tree::conditional::Condition::ThisIsYourTurn(
                                    crate::ability_tree::conditional::ConditionThisIsYourTurn {
                                        span: during_span.merge(turn_span),
                                    },
                                ),
                                span: during_span.merge(turn_span),
                            },
                        )),
                        span: event.span().merge(turn_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
