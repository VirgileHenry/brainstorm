use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "cost: spell ability" makes an activated ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Cost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Colons {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Cost { cost },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Colons { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::Activated(
                        crate::ability_tree::ability::activated::ActivatedAbility {
                            effect: ability.clone(),
                            costs: {
                                let mut costs = crate::utils::HeapArrayVec::new();
                                costs.push(cost.clone());
                                costs
                            },
                            span: cost.span().merge(&ability.span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "cost, cost: spell ability" makes an activated ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Cost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Cost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Colons {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Cost { cost: c1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::Cost { cost: c2 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Colons { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::Activated(
                        crate::ability_tree::ability::activated::ActivatedAbility {
                            effect: ability.clone(),
                            costs: {
                                let mut costs = crate::utils::HeapArrayVec::new();
                                costs.push(c1.clone());
                                costs.push(c2.clone());
                                costs
                            },
                            span: c2.span().merge(&ability.span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "cost, cost, cost: spell ability" makes an activated ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Cost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Cost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Cost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Colons {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Cost { cost: c1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::Cost { cost: c2 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::Cost { cost: c3 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Colons { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::Activated(
                        crate::ability_tree::ability::activated::ActivatedAbility {
                            effect: ability.clone(),
                            costs: {
                                let mut costs = crate::utils::HeapArrayVec::new();
                                costs.push(c1.clone());
                                costs.push(c2.clone());
                                costs.push(c3.clone());
                                costs
                            },
                            span: c3.span().merge(&ability.span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "cost, cost: spell ability" makes an activated ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Cost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Cost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Cost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Cost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Colons {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Cost { cost: c1 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::Cost { cost: c2 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::Cost { cost: c3 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::Cost { cost: c4 },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Colons { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::Activated(
                        crate::ability_tree::ability::activated::ActivatedAbility {
                            effect: ability.clone(),
                            costs: {
                                let mut costs = crate::utils::HeapArrayVec::new();
                                costs.push(c1.clone());
                                costs.push(c2.clone());
                                costs.push(c3.clone());
                                costs.push(c4.clone());
                                costs
                            },
                            span: c4.span().merge(&ability.span),
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
