use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* "<continuous effect>" -> static ability */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[ParserNode::ContinuousEffect {
            effect: Default::default(),
        }
        .id()]),
        merged: ParserNode::WrittenAbility {
            ability: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::ContinuousEffect { effect }] => Ok(ParserNode::WrittenAbility {
                ability: boseiju_tree::ability_tree::ability::WrittenAbility::Static(
                    boseiju_tree::ability_tree::ability::statik::StaticAbility {
                        effect: effect.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: effect.span(),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
