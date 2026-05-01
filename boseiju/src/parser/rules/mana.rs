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
                    mana_cost: terminals::ManaCost {
                        cost: std::iter::once(mana.clone()).collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: mana.node_span(),
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
                    mana_cost: terminals::ManaCost {
                        cost: [m1.clone(), m2.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: m1.node_span().merge(&m2.node_span()),
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
                    mana_cost: terminals::ManaCost {
                        cost: [m1.clone(), m2.clone(), m3.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: m1.node_span().merge(&m3.node_span()),
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
                    mana_cost: terminals::ManaCost {
                        cost: [m1.clone(), m2.clone(), m3.clone(), m4.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: m1.node_span().merge(&m4.node_span()),
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
                    mana_cost: terminals::ManaCost {
                        cost: [m1.clone(), m2.clone(), m3.clone(), m4.clone(), m5.clone()]
                            .into_iter()
                            .collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: m1.node_span().merge(&m5.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
