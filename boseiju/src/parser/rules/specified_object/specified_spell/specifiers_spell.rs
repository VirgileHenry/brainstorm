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
    /* "<spell specifiers> <spell kind>" makes a specified spell */

    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::SpellSpecifiers { specifiers: dummy() }.id(),
            ParserNode::SpellKind { spell: dummy() }.id(),
        ]),
        merged: ParserNode::SpecifiedSpell { spell: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                /* Comment to avoid formmating on the same line */
                ParserNode::SpellSpecifiers { specifiers },
                ParserNode::SpellKind { spell },
            ] => Ok(ParserNode::SpecifiedSpell {
                spell: object::specified_object::SpecifiedSpell {
                    kind: spell.clone(),
                    specifiers: Some(specifiers.clone()),
                    #[cfg(feature = "spanned_tree")]
                    span: specifiers.node_span().merge(&spell.node_span()),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
