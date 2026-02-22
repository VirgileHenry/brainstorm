use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::Mana { mana: dummy() }).id()]),
            result: ParserNode::ManaCost { mana_cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::Mana { mana })] => Some(ParserNode::ManaCost {
                    mana_cost: {
                        let mut cost = arrayvec::ArrayVec::new_const();
                        cost.push(mana.clone());
                        terminals::ManaCost { cost }
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::ManaCost { mana_cost: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::Mana { mana: dummy() }).id(),
            ]),
            result: ParserNode::ManaCost { mana_cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ManaCost { mana_cost },
                    ParserNode::LexerToken(TokenKind::Mana { mana }),
                ] => Some(ParserNode::ManaCost {
                    mana_cost: {
                        let mut mana_cost = mana_cost.clone();
                        if mana_cost.cost.len() == mana_cost.cost.capacity() {
                            /* Safety: avoid the panic if it happens */
                            return None;
                        }
                        mana_cost.cost.push(mana.clone());
                        mana_cost
                    },
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
