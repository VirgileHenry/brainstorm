use super::ParserNode;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Spell ability to ability */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::SpellAbility { ability: dummy() }.id()]),
            result: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpellAbility { ability }] => Some(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Spell(ability.clone())),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Continuous effects are static abilities */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::ContinuousEffect { effect: dummy() }.id()]),
            result: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ContinuousEffect { effect }] => Some(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Static(
                        crate::ability_tree::ability::statik::StaticAbility::ContinuousEffect(effect.clone()),
                    )),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Cost reduction effects are static ailities */
        super::ParserRule {
            from: super::RuleLhs::new(&[ParserNode::CostModificationEffect {
                cost_modification: dummy(),
            }
            .id()]),
            result: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CostModificationEffect { cost_modification }] => Some(ParserNode::Ability {
                    ability: Box::new(crate::ability_tree::ability::Ability::Static(
                        crate::ability_tree::ability::statik::StaticAbility::CostModificationEffect(cost_modification.clone()),
                    )),
                }),
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
