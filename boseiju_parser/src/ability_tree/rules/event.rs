use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [/* "<passive form deed>" is an event */ ParserRule {
        expanded: RuleLhs::new(&[ParserNode::DeedPassiveForm {
            deed: Default::default(),
        }
        .id()]),
        merged: ParserNode::Event {
            event: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::DeedPassiveForm { deed }] => Ok(ParserNode::Event {
                event: boseiju_tree::ability_tree::event::Event {
                    deed: deed.clone(),
                    #[cfg(feature = "spanned_tree")]
                    span: deed.span(),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
