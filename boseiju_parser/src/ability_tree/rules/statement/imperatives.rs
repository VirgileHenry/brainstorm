use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* An imperative list can make a statement. */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[ParserNode::ImperativeList { imperatives: dummy() }.id()]),
        merged: ParserNode::Statement { statement: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::ImperativeList { imperatives }] => Ok(ParserNode::Statement {
                statement: crate::ability_tree::statement::Statement::Imperatives(imperatives.clone()),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
