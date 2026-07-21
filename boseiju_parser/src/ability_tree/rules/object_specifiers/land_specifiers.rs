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
    /* <land subtype> is a land subtype specifier */
    let subtypes_to_specifiers = boseiju_lexer::terminal::LandSubtype::all()
        .map(|subtype| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::LandSubtype(subtype.clone())).id()]),
            merged: ParserNode::LandSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::LandSubtype(subtype))] => Ok(ParserNode::LandSpecifier {
                    specifier: object::specified_object::LandSpecifier::Subtype(object::specified_object::LandSubtypeSpecifier {
                        subtype: subtype.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: subtype.span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let common_specifiers = vec![/* "<control specifier>" is a land specifier */ ParserRule {
        expanded: RuleLhs::new(&[ParserNode::ControlSpecifier {
            specifier: Default::default(),
        }
        .id()]),
        merged: ParserNode::LandSpecifier {
            specifier: Default::default(),
        }
        .id(),
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
            expanded: RuleLhs::new(&[ParserNode::LandSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::LandSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LandSpecifier { specifier }] => Ok(ParserNode::LandSpecifiers {
                    specifiers: object::specified_object::Specifiers::Single(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<land specifier> <land specifier>" -> and list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LandSpecifier {
                    specifier: Default::default(),
                }
                .id(),
                ParserNode::LandSpecifier {
                    specifier: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::LandSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LandSpecifier { specifier: s1 },
                    ParserNode::LandSpecifier { specifier: s2 },
                ] => Ok(ParserNode::LandSpecifiers {
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
        /* "<land specifier> or <land specifier>" -> or list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LandSpecifier {
                    specifier: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LandSpecifier {
                    specifier: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::LandSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LandSpecifier { specifier: s1 },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or { .. })),
                    ParserNode::LandSpecifier { specifier: s2 },
                ] => Ok(ParserNode::LandSpecifiers {
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
