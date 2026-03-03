use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Spell ability to ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::SpellAbility { ability: dummy() }.id()]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpellAbility { ability }] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::Spell(ability.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Static ability kind to static ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::StaticAbilityKind { kind: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::StaticAbilityKind { kind },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { span })),
                ] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::Static(crate::ability_tree::ability::statik::StaticAbility {
                        kind: kind.clone(),
                        condition: None,
                        #[cfg(feature = "spanned_tree")]
                        span: kind.span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Static ability kind with a condition to static ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::StaticAbilityKind { kind: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition { condition: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::StaticAbilityKind { kind },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If { span: if_span })),
                    ParserNode::Condition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { span: dot_span })),
                ] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::Static(crate::ability_tree::ability::statik::StaticAbility {
                        kind: kind.clone(),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                                span: if_span.merge(&condition.span()),
                            },
                        )),
                        span: kind.span().merge(dot_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Static ability kind with a condition before and a comma to static ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition { condition: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    span: Default::default(),
                }))
                .id(),
                ParserNode::StaticAbilityKind { kind: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If { span: if_span })),
                    ParserNode::Condition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::StaticAbilityKind { kind },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { span: dot_span })),
                ] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::Static(crate::ability_tree::ability::statik::StaticAbility {
                        kind: kind.clone(),
                        condition: Some(crate::ability_tree::conditional::Conditional::If(
                            crate::ability_tree::conditional::ConditionalIf {
                                condition: condition.clone(),
                                span: if_span.merge(&condition.span()),
                            },
                        )),
                        span: if_span.merge(dot_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
