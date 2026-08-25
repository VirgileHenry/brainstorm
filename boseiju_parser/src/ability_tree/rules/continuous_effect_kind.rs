use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_tree::ability_tree::continuous_effect;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<replacement effect>" is a continuous effect kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ReplacementEffect {
                effect: Default::default(),
            }
            .id()]),
            merged: ParserNode::ContinuousEffectKind {
                kind: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ReplacementEffect { effect }] => Ok(ParserNode::ContinuousEffectKind {
                    kind: continuous_effect::ContinuousEffectKind::ReplacementEffect(effect.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<object mod effect>" is a continuous effect kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ObjectModsEffect {
                effect: Default::default(),
            }
            .id()]),
            merged: ParserNode::ContinuousEffectKind {
                kind: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ObjectModsEffect { effect }] => Ok(ParserNode::ContinuousEffectKind {
                    kind: continuous_effect::ContinuousEffectKind::ObjectModEffect(effect.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
