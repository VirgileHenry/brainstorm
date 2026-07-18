use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate::NumberOperation;
use boseiju_tree::ability_tree::ability::statik::continuous_effect::continuous_effect_kind;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "+<number>/+<nmber>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::PowerToughnessModifiers {
                modifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Plus {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol { .. })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Plus { .. })),
                    ParserNode::Number { number: toughness },
                ] => Ok(ParserNode::PowerToughnessModifiers {
                    modifiers: continuous_effect_kind::PowerToughnessModifiers::PlusPlus(
                        continuous_effect_kind::PowerToughnessModifiersPlusPlus {
                            power_mod: power.clone(),
                            toughness_mod: toughness.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: toughness.span().merge(start_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "+<number>/-<nmber>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Minus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::PowerToughnessModifiers {
                modifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Plus {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol { .. })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Minus { .. })),
                    ParserNode::Number { number: toughness },
                ] => Ok(ParserNode::PowerToughnessModifiers {
                    modifiers: continuous_effect_kind::PowerToughnessModifiers::PlusMinus(
                        continuous_effect_kind::PowerToughnessModifiersPlusMinus {
                            power_mod: power.clone(),
                            toughness_mod: toughness.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: toughness.span().merge(start_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "-<number>/+<nmber>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Minus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::PowerToughnessModifiers {
                modifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Minus {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol { .. })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Plus { .. })),
                    ParserNode::Number { number: toughness },
                ] => Ok(ParserNode::PowerToughnessModifiers {
                    modifiers: continuous_effect_kind::PowerToughnessModifiers::MinusPlus(
                        continuous_effect_kind::PowerToughnessModifiersMinusPlus {
                            power_mod: power.clone(),
                            toughness_mod: toughness.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: toughness.span().merge(start_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "-<number>/-<nmber>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Minus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Minus {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number {
                    number: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::PowerToughnessModifiers {
                modifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Minus {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Number { number: power },
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol { .. })),
                    ParserNode::LexerToken(Token::NumberOperation(NumberOperation::Minus { .. })),
                    ParserNode::Number { number: toughness },
                ] => Ok(ParserNode::PowerToughnessModifiers {
                    modifiers: continuous_effect_kind::PowerToughnessModifiers::MinusMinus(
                        continuous_effect_kind::PowerToughnessModifiersMinusMinus {
                            power_mod: power.clone(),
                            toughness_mod: toughness.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: toughness.span().merge(start_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
