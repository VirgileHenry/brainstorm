use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* Some deeds in the passive form can be costs */
    [/* "<passive tap deed>" is an atomic cost */ ParserRule {
        expanded: RuleLhs::new(&[ParserNode::TapPassive {
            deed: Default::default(),
        }
        .id()]),
        merged: ParserNode::AtomicCost {
            cost: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::TapPassive { deed }] => Ok(ParserNode::AtomicCost {
                cost: boseiju_tree::ability_tree::cost::AtomicCost::Tap(deed.clone()),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
