use crate::ability_tree::object;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<specified card>" can be used as a card kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedCard { card: dummy() }.id()]),
            merged: ParserNode::CardKind { card: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedCard { card }] => Ok(ParserNode::CardKind {
                    card: object::kind::CardKind::Specified(card.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent kind>" can be used as a card kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::PermanentKind { permanent: dummy() }.id()]),
            merged: ParserNode::CardKind { card: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::PermanentKind { permanent }] => Ok(ParserNode::CardKind {
                    card: object::kind::CardKind::Permanent(permanent.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
