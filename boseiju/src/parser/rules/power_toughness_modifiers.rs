use crate::ability_tree::ability::statik::continuous_effect::*;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates::PowerToughnessModElements;
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
        /* "+<number>/+<nmber>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::PowerToughnessModifiers { modifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                    ParserNode::Number { number: toughness },
                ] => Ok(ParserNode::PowerToughnessModifiers {
                    modifiers: PowerToughnessModifiers::PlusPlus(PowerToughnessModifiersPlusPlus {
                        power_mod: power.clone(),
                        toughness_mod: toughness.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: toughness.node_span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "+<number>/-<nmber>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Minus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::PowerToughnessModifiers { modifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Minus { .. })),
                    ParserNode::Number { number: toughness },
                ] => Ok(ParserNode::PowerToughnessModifiers {
                    modifiers: PowerToughnessModifiers::PlusMinus(PowerToughnessModifiersPlusMinus {
                        power_mod: power.clone(),
                        toughness_mod: toughness.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: toughness.node_span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "-<number>/+<nmber>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Minus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::PowerToughnessModifiers { modifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Minus {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                    ParserNode::Number { number: toughness },
                ] => Ok(ParserNode::PowerToughnessModifiers {
                    modifiers: PowerToughnessModifiers::MinusPlus(PowerToughnessModifiersMinusPlus {
                        power_mod: power.clone(),
                        toughness_mod: toughness.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: toughness.node_span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "-<number>/-<nmber>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Minus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Minus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::PowerToughnessModifiers { modifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Minus {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                    ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Minus { .. })),
                    ParserNode::Number { number: toughness },
                ] => Ok(ParserNode::PowerToughnessModifiers {
                    modifiers: PowerToughnessModifiers::MinusMinus(PowerToughnessModifiersMinusMinus {
                        power_mod: power.clone(),
                        toughness_mod: toughness.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: toughness.node_span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
