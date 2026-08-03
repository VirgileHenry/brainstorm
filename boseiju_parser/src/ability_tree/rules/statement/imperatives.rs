use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* An imperative list can make a statement. */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[ParserNode::ImperativeList {
            imperatives: Default::default(),
        }
        .id()]),
        merged: ParserNode::Statement {
            statement: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::ImperativeList { imperatives }] => Ok(ParserNode::Statement {
                statement: boseiju_tree::ability_tree::statement::Statement::Imperatives(imperatives.clone()),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
