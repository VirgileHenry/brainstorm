use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = ParserRule> {
    let common_specifiers = vec![
        /* "<control specifier>" is a creature specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ControlSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::PermanentSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ControlSpecifier { specifier }] => Ok(ParserNode::PermanentSpecifier {
                    specifier: crate::ability_tree::object::specified_object::PermanentSpecifier::Control(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<color specifier>" is a creature specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ColorSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::PermanentSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ColorSpecifier { specifier }] => Ok(ParserNode::PermanentSpecifier {
                    specifier: crate::ability_tree::object::specified_object::PermanentSpecifier::Color(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [common_specifiers].into_iter().flatten()
}
