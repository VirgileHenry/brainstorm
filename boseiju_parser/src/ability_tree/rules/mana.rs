use super::ParserNode;
use boseiju_lexer::Token;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id()]),
            merged: ParserNode::ManaCost {
                mana_cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::ManaSymbol(mana_symbol))] => Ok(ParserNode::ManaCost {
                    mana_cost: boseiju_tree::ability_tree::mana_cost::ManaCost {
                        symbols: std::iter::once(mana_symbol.clone()).collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: mana_symbol.span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
            ]),
            merged: ParserNode::ManaCost {
                mana_cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ManaSymbol(m1)),
                    ParserNode::LexerToken(Token::ManaSymbol(m2)),
                ] => Ok(ParserNode::ManaCost {
                    mana_cost: boseiju_tree::ability_tree::mana_cost::ManaCost {
                        symbols: [m1.clone(), m2.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: m1.span().merge(&m2.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
            ]),
            merged: ParserNode::ManaCost {
                mana_cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ManaSymbol(m1)),
                    ParserNode::LexerToken(Token::ManaSymbol(m2)),
                    ParserNode::LexerToken(Token::ManaSymbol(m3)),
                ] => Ok(ParserNode::ManaCost {
                    mana_cost: boseiju_tree::ability_tree::mana_cost::ManaCost {
                        symbols: [m1.clone(), m2.clone(), m3.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: m1.span().merge(&m3.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
            ]),
            merged: ParserNode::ManaCost {
                mana_cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ManaSymbol(m1)),
                    ParserNode::LexerToken(Token::ManaSymbol(m2)),
                    ParserNode::LexerToken(Token::ManaSymbol(m3)),
                    ParserNode::LexerToken(Token::ManaSymbol(m4)),
                ] => Ok(ParserNode::ManaCost {
                    mana_cost: boseiju_tree::ability_tree::mana_cost::ManaCost {
                        symbols: [m1.clone(), m2.clone(), m3.clone(), m4.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: m1.span().merge(&m4.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
                ParserNode::LexerToken(Token::ManaSymbol(Default::default())).id(),
            ]),
            merged: ParserNode::ManaCost {
                mana_cost: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ManaSymbol(m1)),
                    ParserNode::LexerToken(Token::ManaSymbol(m2)),
                    ParserNode::LexerToken(Token::ManaSymbol(m3)),
                    ParserNode::LexerToken(Token::ManaSymbol(m4)),
                    ParserNode::LexerToken(Token::ManaSymbol(m5)),
                ] => Ok(ParserNode::ManaCost {
                    mana_cost: boseiju_tree::ability_tree::mana_cost::ManaCost {
                        symbols: [m1.clone(), m2.clone(), m3.clone(), m4.clone(), m5.clone()]
                            .into_iter()
                            .collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: m1.span().merge(&m5.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
