use crate::ability_tree::object;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<count> <specified creature>" is a creature */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                ParserNode::SpecifiedCreature { creature: dummy() }.id(),
            ]),
            merged: ParserNode::Creature { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::CountSpecifier { count },
                    ParserNode::SpecifiedCreature { creature },
                ] => Ok(ParserNode::Creature {
                    creature: object::Creature::Reference(object::reference::CreatureReference {
                        count: count.clone(),
                        creature: creature.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.node_span().merge(&creature.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified creature>" is a creature with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedCreature { creature: dummy() }.id()]),
            merged: ParserNode::Creature { creature: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedCreature { creature }] => Ok(ParserNode::Creature {
                    creature: object::Creature::Reference(object::reference::CreatureReference {
                        count: object::CountSpecifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: creature.node_span().empty_at_start(),
                        },
                        creature: creature.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
