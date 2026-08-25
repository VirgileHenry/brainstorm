use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_tree::ability_tree::object;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<creature kind>" can be used as a damage receiver kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedCreature {
                creature: Default::default(),
            }
            .id()]),
            merged: ParserNode::DamageReceiverKind {
                receiver: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedCreature { creature }] => Ok(ParserNode::DamageReceiverKind {
                    receiver: object::kind::DamageReceiverKind::Creature(creature.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<planeswalker kind>" can be used as a damage receiver kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedPlaneswalker {
                planeswalker: Default::default(),
            }
            .id()]),
            merged: ParserNode::DamageReceiverKind {
                receiver: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedPlaneswalker { planeswalker }] => Ok(ParserNode::DamageReceiverKind {
                    receiver: object::kind::DamageReceiverKind::Planeswalker(planeswalker.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<player specifier>" can be used as a damage receiver kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::PlayerActive {
                player: Default::default(),
            }
            .id()]),
            merged: ParserNode::DamageReceiverKind {
                receiver: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::PlayerActive { player }] => Ok(ParserNode::DamageReceiverKind {
                    receiver: object::kind::DamageReceiverKind::Player(player.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
