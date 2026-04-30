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
    /* "<permanent kind>" is the default specified permanent */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[ParserNode::PermanentKind { permanent: dummy() }.id()]),
        merged: ParserNode::SpecifiedPermanent { permanent: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::PermanentKind { permanent }] => Ok(ParserNode::SpecifiedPermanent {
                permanent: object::specified_object::SpecifiedPermanent {
                    kind: permanent.clone(),
                    specifiers: None,
                    #[cfg(feature = "spanned_tree")]
                    span: permanent.node_span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
