use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = ParserRule> {
    /* <artifact subtype> is a artifact "subtype" specifier */
    let subtypes_to_specifiers = crate::ability_tree::terminals::ArtifactSubtype::all()
        .map(|subtype| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::ArtifactSubtype(subtype.clone())).id()]),
            merged: ParserNode::ArtifactSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::ArtifactSubtype(subtype))] => Ok(ParserNode::ArtifactSpecifier {
                    specifier: object::specified_object::ArtifactSpecifier::Subtype(
                        object::specified_object::ArtifactSubtypeSpecifier {
                            subtype: subtype.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: subtype.node_span(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let common_specifiers = vec![
        /* "<control specifier>" is a artifact specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ControlSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::ArtifactSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ControlSpecifier { specifier }] => Ok(ParserNode::ArtifactSpecifier {
                    specifier: object::specified_object::ArtifactSpecifier::Control(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<color specifier>" is a artifact specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ColorSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::ArtifactSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ColorSpecifier { specifier }] => Ok(ParserNode::ArtifactSpecifier {
                    specifier: object::specified_object::ArtifactSpecifier::Color(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<another specifier>" is a artifact specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::AnotherSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::ArtifactSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::AnotherSpecifier { specifier }] => Ok(ParserNode::ArtifactSpecifier {
                    specifier: object::specified_object::ArtifactSpecifier::Another(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    let merging_specifiers = vec![
        /* "<artifact specifier>" on its own can make a artifact specifiers node */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ArtifactSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::ArtifactSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ArtifactSpecifier { specifier }] => Ok(ParserNode::ArtifactSpecifiers {
                    specifiers: object::specified_object::Specifiers::Single(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [subtypes_to_specifiers, common_specifiers, merging_specifiers]
        .into_iter()
        .flatten()
}
