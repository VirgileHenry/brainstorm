use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::Mana { mana: dummy() }).id()]),
            merged: ParserNode::ManaCost { mana_cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::Mana { mana })] => Ok(ParserNode::ManaCost {
                    mana_cost: {
                        let mut cost = arrayvec::ArrayVec::new_const();
                        cost.push(mana.clone());
                        terminals::ManaCost {
                            cost,
                            #[cfg(feature = "spanned_tree")]
                            span: mana.node_span(),
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::ManaCost { mana_cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Mana { mana: m1 }),
                    ParserNode::LexerToken(Token::Mana { mana: m2 }),
                ] => Ok(ParserNode::ManaCost {
                    mana_cost: {
                        let mut cost = arrayvec::ArrayVec::new_const();
                        cost.push(m1.clone());
                        cost.push(m2.clone());
                        terminals::ManaCost {
                            cost,
                            #[cfg(feature = "spanned_tree")]
                            span: m1.node_span().merge(&m2.node_span()),
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::ManaCost { mana_cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Mana { mana: m1 }),
                    ParserNode::LexerToken(Token::Mana { mana: m2 }),
                    ParserNode::LexerToken(Token::Mana { mana: m3 }),
                ] => Ok(ParserNode::ManaCost {
                    mana_cost: {
                        let mut cost = arrayvec::ArrayVec::new_const();
                        cost.push(m1.clone());
                        cost.push(m2.clone());
                        cost.push(m3.clone());
                        terminals::ManaCost {
                            cost,
                            #[cfg(feature = "spanned_tree")]
                            span: m1.node_span().merge(&m3.node_span()),
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::ManaCost { mana_cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Mana { mana: m1 }),
                    ParserNode::LexerToken(Token::Mana { mana: m2 }),
                    ParserNode::LexerToken(Token::Mana { mana: m3 }),
                    ParserNode::LexerToken(Token::Mana { mana: m4 }),
                ] => Ok(ParserNode::ManaCost {
                    mana_cost: {
                        let mut cost = arrayvec::ArrayVec::new_const();
                        cost.push(m1.clone());
                        cost.push(m2.clone());
                        cost.push(m3.clone());
                        cost.push(m4.clone());
                        terminals::ManaCost {
                            cost,
                            #[cfg(feature = "spanned_tree")]
                            span: m1.node_span().merge(&m4.node_span()),
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::ManaCost { mana_cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Mana { mana: m1 }),
                    ParserNode::LexerToken(Token::Mana { mana: m2 }),
                    ParserNode::LexerToken(Token::Mana { mana: m3 }),
                    ParserNode::LexerToken(Token::Mana { mana: m4 }),
                    ParserNode::LexerToken(Token::Mana { mana: m5 }),
                ] => Ok(ParserNode::ManaCost {
                    mana_cost: {
                        let mut cost = arrayvec::ArrayVec::new_const();
                        cost.push(m1.clone());
                        cost.push(m2.clone());
                        cost.push(m3.clone());
                        cost.push(m4.clone());
                        cost.push(m5.clone());
                        terminals::ManaCost {
                            cost,
                            #[cfg(feature = "spanned_tree")]
                            span: m1.node_span().merge(&m5.node_span()),
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
