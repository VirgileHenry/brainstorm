use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_tree::ability_tree::number;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* <permanent reference> can make a number of permanents on the battlefield */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::PermanentPassive {
                permanent: Default::default(),
            }
            .id()]),
            merged: ParserNode::GameStateNumber {
                number: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::PermanentPassive { permanent }] => Ok(ParserNode::GameStateNumber {
                    number: number::GameStateNumber::NumberOfPermanents(number::NumberOfPermanents {
                        permanent: permanent.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: permanent.span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
