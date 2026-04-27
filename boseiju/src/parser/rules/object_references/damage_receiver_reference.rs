use crate::ability_tree::object;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<creature reference>" can be used as a damage receiver reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::CreatureReference { creature: dummy() }.id()]),
            merged: ParserNode::DamageReceiverReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CreatureReference { creature }] => Ok(ParserNode::DamageReceiverReference {
                    reference: object::DamageReceiverReference::Creature(creature.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<player specifier>" can be used as a damage receiver reference */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::Player { player: dummy() }.id()]),
            merged: ParserNode::DamageReceiverReference { reference: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Player { player }] => Ok(ParserNode::DamageReceiverReference {
                    reference: object::DamageReceiverReference::Player(player.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
