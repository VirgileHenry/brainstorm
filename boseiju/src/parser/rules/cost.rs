use super::ParserNode;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* A mana cost can make a cost */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::ManaCost { mana_cost: dummy() }.id()]),
            merged: ParserNode::Cost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ManaCost { mana_cost }] => Ok(ParserNode::Cost {
                    cost: crate::ability_tree::cost::Cost::ManaCost(mana_cost.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* An imperative can make a cost */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Imperative { imperative: dummy() }.id()]),
            merged: ParserNode::Cost { cost: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Imperative { imperative }] => Ok(ParserNode::Cost {
                    cost: crate::ability_tree::cost::Cost::Imperative(imperative.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
