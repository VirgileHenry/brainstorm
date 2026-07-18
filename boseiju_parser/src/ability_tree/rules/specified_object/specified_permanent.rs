use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_tree::ability_tree::object;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* "<permanent kind>" is the default specified permanent */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[ParserNode::PermanentKind {
            permanent: Default::default(),
        }
        .id()]),
        merged: ParserNode::SpecifiedPermanent {
            permanent: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::PermanentKind { permanent }] => Ok(ParserNode::SpecifiedPermanent {
                permanent: object::specified_object::SpecifiedPermanent {
                    kind: permanent.clone(),
                    specifiers: None,
                    #[cfg(feature = "spanned_tree")]
                    span: permanent.span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
