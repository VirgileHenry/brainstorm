use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "with mana value <number>" makes a mana value property specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::ManaValue {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::ManaValue { .. })),
                    ParserNode::Number { number },
                ] => Ok(ParserNode::ObjectSpecifier {
                    specifier: object::ObjectSpecifier::WithProperty(object::CardPropertySpecifier::CardManaValue(
                        object::CardManaValuePropertySpecifier {
                            mana_value: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.node_span().merge(start_span),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "with total mana value <number>" makes a total mana value property specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Total {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::ManaValue {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Total { .. })),
                    ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::ManaValue { .. })),
                    ParserNode::Number { number },
                ] => Ok(ParserNode::ObjectSpecifier {
                    specifier: object::ObjectSpecifier::WithProperty(object::CardPropertySpecifier::CardTotalManaValue(
                        object::CardTotalManaValuePropertySpecifier {
                            total_mana_value: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.node_span().merge(start_span),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "with power <number>" makes a power property specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::Power {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::Power { .. })),
                    ParserNode::Number { number },
                ] => Ok(ParserNode::ObjectSpecifier {
                    specifier: object::ObjectSpecifier::WithProperty(object::CardPropertySpecifier::CardPower(
                        object::CardPowerPropertySpecifier {
                            power: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.node_span().merge(start_span),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "with total power <number>" makes a total power property specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Total {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::Power {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Total { .. })),
                    ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::Power { .. })),
                    ParserNode::Number { number },
                ] => Ok(ParserNode::ObjectSpecifier {
                    specifier: object::ObjectSpecifier::WithProperty(object::CardPropertySpecifier::CardTotalPower(
                        object::CardTotalPowerPropertySpecifier {
                            total_power: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.node_span().merge(start_span),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "with total power and toughness <number>" makes a total power and toughness property specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Total {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::Power {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::Toughness {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Total { .. })),
                    ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::Power { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::And { .. })),
                    ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::Toughness { .. })),
                    ParserNode::Number { number },
                ] => Ok(ParserNode::ObjectSpecifier {
                    specifier: object::ObjectSpecifier::WithProperty(object::CardPropertySpecifier::CardTotalPowerAndToughness(
                        object::CardTotalPowerAndToughnessPropertySpecifier {
                            total_power_and_toughness: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.node_span().merge(start_span),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "with toughness <number>" makes a toughness property specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::Toughness {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
            ]),
            merged: ParserNode::ObjectSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::With {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::CardProperty(terminals::CardProperty::Toughness { .. })),
                    ParserNode::Number { number },
                ] => Ok(ParserNode::ObjectSpecifier {
                    specifier: object::ObjectSpecifier::WithProperty(object::CardPropertySpecifier::CardToughness(
                        object::CardToughnessPropertySpecifier {
                            toughness: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: number.node_span().merge(start_span),
                        },
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
