use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* An imperative can make a cost */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[ParserNode::Imperative {
            imperative: Default::default(),
        }
        .id()]),
        merged: ParserNode::ImperativeAsCost {
            cost: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::Imperative { imperative }] => Ok(ParserNode::ImperativeAsCost {
                cost: imperative.clone(),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
