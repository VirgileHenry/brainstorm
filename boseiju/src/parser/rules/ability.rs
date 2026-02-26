use super::ParserNode;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Spell ability to ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::SpellAbility { ability: dummy() }.id()]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpellAbility { ability }] => Ok(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Spell(ability.clone())),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Continuous effects are static abilities */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::ContinuousEffect { effect: dummy() }.id()]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ContinuousEffect { effect }] => Ok(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Static(
                        crate::ability_tree::ability::statik::StaticAbility::ContinuousEffect(effect.clone()),
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Cost reduction effects are static ailities */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::CostModificationEffect {
                cost_modification: dummy(),
            }
            .id()]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CostModificationEffect { cost_modification }] => Ok(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Static(
                        crate::ability_tree::ability::statik::StaticAbility::CostModificationEffect(cost_modification.clone()),
                    )),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
