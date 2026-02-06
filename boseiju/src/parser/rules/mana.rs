use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::parser::node::DummyInit;
use idris::Idris;

fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

pub fn rules() -> impl Iterator<Item = super::ParserRule> {
    [
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::Mana { mana: dummy() }).id()]),
            result: ParserNode::ManaCost { mana_cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::Mana { mana })] => Some(ParserNode::ManaCost {
                    mana_cost: {
                        let mut mana_cost = arrayvec::ArrayVec::new();
                        mana_cost.push(mana.clone());
                        terminals::ManaCost(mana_cost)
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
                        if mana_cost.0.len() == mana_cost.0.capacity() {
                            /* Safety: avoid the panic if it happens */
                            return None;
                        }
                        mana_cost.0.push(mana.clone());
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
