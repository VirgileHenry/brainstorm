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
                ParserNode::ManaCost { mana_cost: dummy() }.id(),
                ParserNode::LexerToken(Token::Mana { mana: dummy() }).id(),
            ]),
            merged: ParserNode::ManaCost { mana_cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ManaCost { mana_cost },
                    ParserNode::LexerToken(Token::Mana { mana }),
                ] => Ok(ParserNode::ManaCost {
                    mana_cost: {
                        let mut mana_cost = mana_cost.clone();
                        if mana_cost.cost.try_push(mana.clone()).is_err() {
                            return Err("Too many mana symbols for cost !");
                        }
                        mana_cost
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
