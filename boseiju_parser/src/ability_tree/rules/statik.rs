mod statik_ability_kind;

use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    let statik_ability_rules = vec![
        /* "<static ab kind>" -> static ability */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::StaticAbilityKind {
                kind: Default::default(),
            }
            .id()]),
            merged: ParserNode::WrittenAbility {
                ability: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::StaticAbilityKind { kind }] => Ok(ParserNode::WrittenAbility {
                    ability: boseiju_tree::ability_tree::ability::WrittenAbility::Static(
                        boseiju_tree::ability_tree::ability::statik::StaticAbility {
                            kind: kind.clone(),
                            condition: None,
                            #[cfg(feature = "spanned_tree")]
                            span: kind.span(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "as long as <condition>, <static ab kind>" -> static ability */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::AsLongAs {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Condition {
                    condition: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::StaticAbilityKind {
                    kind: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::WrittenAbility {
                ability: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::AsLongAs {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Condition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::StaticAbilityKind { kind },
                ] => Ok(ParserNode::WrittenAbility {
                    ability: boseiju_tree::ability_tree::ability::WrittenAbility::Static(
                        boseiju_tree::ability_tree::ability::statik::StaticAbility {
                            kind: kind.clone(),
                            condition: Some(boseiju_tree::ability_tree::conditional::Conditional::If(
                                boseiju_tree::ability_tree::conditional::ConditionalIf {
                                    condition: condition.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: condition.span().merge(start_span),
                                },
                            )),
                            #[cfg(feature = "spanned_tree")]
                            span: kind.span().merge(start_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<static ab kind> if <condition>" -> static ability */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::StaticAbilityKind {
                    kind: Default::default(),
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
                    ParserNode::StaticAbilityKind { kind },
                    ParserNode::LexerToken(Token::EnglishConditional(intermediate::EnglishConditional::If {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Condition { condition },
                ] => Ok(ParserNode::WrittenAbility {
                    ability: boseiju_tree::ability_tree::ability::WrittenAbility::Static(
                        boseiju_tree::ability_tree::ability::statik::StaticAbility {
                            kind: kind.clone(),
                            condition: Some(boseiju_tree::ability_tree::conditional::Conditional::If(
                                boseiju_tree::ability_tree::conditional::ConditionalIf {
                                    condition: condition.clone(),
                                    #[cfg(feature = "spanned_tree")]
                                    span: condition.span().merge(start_span),
                                },
                            )),
                            #[cfg(feature = "spanned_tree")]
                            span: kind.span().merge(&condition.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [statik_ability_rules, statik_ability_kind::rules().collect::<Vec<_>>()]
        .into_iter()
        .flatten()
}
