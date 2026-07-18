use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_tree::ability_tree::object;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* "<spell specifiers> <spell kind>" makes a specified spell */

    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::SpellSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            ParserNode::SpellKind {
                spell: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::SpecifiedSpell {
            spell: Default::default(),
        }
        .id(),
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
                    span: specifiers.span().merge(&spell.span()),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
