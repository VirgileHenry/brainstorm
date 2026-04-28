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
    /* <land subtype> is a land subtype specifier */
    let subtypes_to_specifiers = crate::ability_tree::terminals::LandSubtype::all()
        .map(|subtype| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::LandSubtype(subtype.clone())).id()]),
            merged: ParserNode::LandSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::LandSubtype(subtype))] => Ok(ParserNode::LandSpecifier {
                    specifier: object::specified_object::LandSpecifier::Subtype(object::specified_object::LandSubtypeSpecifier {
                        subtype: subtype.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: subtype.node_span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let common_specifiers = vec![/* "<control specifier>" is a land specifier */ ParserRule {
        expanded: RuleLhs::new(&[ParserNode::ControlSpecifier { specifier: dummy() }.id()]),
        merged: ParserNode::LandSpecifier { specifier: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::ControlSpecifier { specifier }] => Ok(ParserNode::LandSpecifier {
                specifier: object::specified_object::LandSpecifier::Control(specifier.clone()),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }];

    let merging_specifiers = vec![
        /* "<land specifier>" on its own can make a land specifiers node */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LandSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::LandSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LandSpecifier { specifier }] => Ok(ParserNode::LandSpecifiers {
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
