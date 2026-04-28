use crate::ability_tree::object;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<creature kind>" can be used as a damage receiver kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::CreatureKind { creature: dummy() }.id()]),
            merged: ParserNode::DamageReceiverKind { receiver: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CreatureKind { creature }] => Ok(ParserNode::DamageReceiverKind {
                    receiver: object::kind::DamageReceiverKind::Creature(creature.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<planeswalker kind>" can be used as a damage receiver kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::PlaneswalkerKind { planeswalker: dummy() }.id()]),
            merged: ParserNode::DamageReceiverKind { receiver: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::PlaneswalkerKind { planeswalker }] => Ok(ParserNode::DamageReceiverKind {
                    receiver: object::kind::DamageReceiverKind::Planeswalker(planeswalker.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<player specifier>" can be used as a damage receiver kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::Player { player: dummy() }.id()]),
            merged: ParserNode::DamageReceiverKind { receiver: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Player { player }] => Ok(ParserNode::DamageReceiverKind {
                    receiver: object::kind::DamageReceiverKind::Player(player.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
