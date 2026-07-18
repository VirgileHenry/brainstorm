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
    let common_specifiers = vec![
        /* "<control specifier>" is a permanent specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ControlSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::PermanentSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ControlSpecifier { specifier }] => Ok(ParserNode::PermanentSpecifier {
                    specifier: boseiju_tree::ability_tree::object::specified_object::PermanentSpecifier::Control(
                        specifier.clone(),
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<color specifier>" is a permanent specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ColorSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::PermanentSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ColorSpecifier { specifier }] => Ok(ParserNode::PermanentSpecifier {
                    specifier: boseiju_tree::ability_tree::object::specified_object::PermanentSpecifier::Color(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    let merging_specifiers = vec![
        /* "<permanent specifier>" on its own can make a permanent specifiers node */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::PermanentSpecifier {
                specifier: Default::default(),
            }
            .id()]),
            merged: ParserNode::PermanentSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::PermanentSpecifier { specifier }] => Ok(ParserNode::PermanentSpecifiers {
                    specifiers: object::specified_object::Specifiers::Single(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<permanent specifier> <permanent specifier>" -> and list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PermanentSpecifier {
                    specifier: Default::default(),
                }
                .id(),
                ParserNode::PermanentSpecifier {
                    specifier: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::PermanentSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PermanentSpecifier { specifier: s1 },
                    ParserNode::PermanentSpecifier { specifier: s2 },
                ] => Ok(ParserNode::PermanentSpecifiers {
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
        /* "<permanent specifier> or <permanent specifier>" -> or list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::PermanentSpecifier {
                    specifier: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::PermanentSpecifier {
                    specifier: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::PermanentSpecifiers {
                specifiers: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::PermanentSpecifier { specifier: s1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Or { .. })),
                    ParserNode::PermanentSpecifier { specifier: s2 },
                ] => Ok(ParserNode::PermanentSpecifiers {
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

    [common_specifiers, merging_specifiers].into_iter().flatten()
}
