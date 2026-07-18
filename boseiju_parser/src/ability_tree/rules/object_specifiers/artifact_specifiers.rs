use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::object;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = ParserRule> {
    /* <artifact subtype> is a artifact "subtype" specifier */
    let subtypes_to_specifiers = boseiju_lexer::terminal::ArtifactSubtype::all()
        .map(|subtype| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::ArtifactSubtype(subtype.clone())).id()]),
            merged: ParserNode::ArtifactSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::ArtifactSubtype(subtype))] => Ok(ParserNode::ArtifactSpecifier {
                    specifier: object::specified_object::ArtifactSpecifier::Subtype(
                        object::specified_object::ArtifactSubtypeSpecifier {
                            subtype: subtype.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: subtype.span(),
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
            expanded: RuleLhs::new(&[ParserNode::ControlSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::ArtifactSpecifier {
                specifier: Default::default(),
            }
            .id(),
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
            expanded: RuleLhs::new(&[ParserNode::ColorSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::ArtifactSpecifier {
                specifier: Default::default(),
            }
            .id(),
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
            expanded: RuleLhs::new(&[ParserNode::AnotherSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::ArtifactSpecifier {
                specifier: Default::default(),
            }
            .id(),
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
            expanded: RuleLhs::new(&[ParserNode::ArtifactSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::ArtifactSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
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
                ParserNode::ArtifactSpecifier {
                    specifier: Default::default(),
                }
                .id(),
                ParserNode::ArtifactSpecifier {
                    specifier: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ArtifactSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ArtifactSpecifier { specifier: s1 },
                    ParserNode::ArtifactSpecifier { specifier: s2 },
                ] => Ok(ParserNode::ArtifactSpecifiers {
                    specifiers: object::specified_object::Specifiers::And(object::specified_object::SpecifierAndList {
                        specifiers: [s1.clone(), s2.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: s1.span().merge(&s2.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<artifact specifier> or <artifact specifier>" -> or list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ArtifactSpecifier {
                    specifier: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ArtifactSpecifier {
                    specifier: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ArtifactSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ArtifactSpecifier { specifier: s1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Or { .. })),
                    ParserNode::ArtifactSpecifier { specifier: s2 },
                ] => Ok(ParserNode::ArtifactSpecifiers {
                    specifiers: object::specified_object::Specifiers::Or(object::specified_object::SpecifierOrList {
                        specifiers: [s1.clone(), s2.clone()].into_iter().collect(),
                        #[cfg(feature = "spanned_tree")]
                        span: s1.span().merge(&s2.span()),
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
