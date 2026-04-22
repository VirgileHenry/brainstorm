use super::ParserNode;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<statement>" is a spell ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Statement { statement: dummy() }.id()]),
            merged: ParserNode::SpellAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Statement { statement }] => Ok(ParserNode::SpellAbility {
                    ability: {
                        let mut statements = crate::utils::HeapArrayVec::new();
                        statements.push(statement.clone());
                        crate::ability_tree::ability::spell::SpellAbility {
                            effects: statements,
                            #[cfg(feature = "spanned_tree")]
                            span: statement.node_span(),
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<spell ability>" makes an ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::SpellAbility { ability: dummy() }.id()]),
            merged: ParserNode::WrittenAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpellAbility { ability }] => Ok(ParserNode::WrittenAbility {
                    ability: crate::ability_tree::ability::WrittenAbility::Spell(
                        crate::ability_tree::ability::spell::SpellAbility {
                            effects: ability.effects.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: ability.node_span(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
