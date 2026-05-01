use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
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
        /* "<artifact specifier> <artifact specifier>" -> and list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ArtifactSpecifier { specifier: dummy() }.id(),
                ParserNode::ArtifactSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::ArtifactSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ArtifactSpecifier { specifier: s1 },
                    ParserNode::ArtifactSpecifier { specifier: s2 },
                ] => Ok(ParserNode::ArtifactSpecifiers {
                    specifiers: object::specified_object::Specifiers::And(object::specified_object::SpecifierAndList {
                        specifiers: [s1.clone(), s2.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: s1.node_span().merge(&s2.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<artifact specifier> or <artifact specifier>" -> or list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ArtifactSpecifier { specifier: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ArtifactSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::ArtifactSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ArtifactSpecifier { specifier: s1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::ArtifactSpecifier { specifier: s2 },
                ] => Ok(ParserNode::ArtifactSpecifiers {
                    specifiers: object::specified_object::Specifiers::Or(object::specified_object::SpecifierOrList {
                        specifiers: [s1.clone(), s2.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: s1.node_span().merge(&s2.node_span()),
                    }),
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
