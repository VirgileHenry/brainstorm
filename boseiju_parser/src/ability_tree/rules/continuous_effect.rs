use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<continuous effect kind>" is a continuous effect on its own */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ContinuousEffectKind {
                kind: Default::default(),
            }
            .id()]),
            merged: ParserNode::ContinuousEffect {
                effect: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ContinuousEffectKind { kind }] => Ok(ParserNode::ContinuousEffect {
                    effect: boseiju_tree::ability_tree::continuous_effect::ContinuousEffect {
                        kind: kind.clone(),
                        condition: None,
                        #[cfg(feature = "spanned_tree")]
                        span: kind.span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "as long as <condition>, <continuous effect kind>" is a continuous effect with a condition */
    ]
    .into_iter()
}
