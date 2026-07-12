use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* An imperative can make a cost */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[ParserNode::Imperative { imperative: dummy() }.id()]),
        merged: ParserNode::ImperativeAsCost { cost: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::Imperative { imperative }] => Ok(ParserNode::ImperativeAsCost {
                cost: imperative.clone(),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
