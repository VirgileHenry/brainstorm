use crate::ability_tree::number;
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
        /* <permanent reference> can make a number of permanents on the battlefield */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::PermanentReference { permanent: dummy() }.id()]),
            merged: ParserNode::GameStateNumber { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::PermanentReference { permanent }] => Ok(ParserNode::GameStateNumber {
                    number: number::GameStateNumber::NumberOfPermanents(number::NumberOfPermanents {
                        permanent: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.node_span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
