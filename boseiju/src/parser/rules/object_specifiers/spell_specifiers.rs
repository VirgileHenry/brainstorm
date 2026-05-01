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
    let common_specifiers = vec![
        /* "<color specifier>" is a spell specifier */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::ColorSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::SpellSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ColorSpecifier { specifier }] => Ok(ParserNode::SpellSpecifier {
                    specifier: object::specified_object::SpellSpecifier::Color(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<spell specifier>" on its own can make a spell specifiers node */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpellSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::SpellSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpellSpecifier { specifier }] => Ok(ParserNode::SpellSpecifiers {
                    specifiers: object::specified_object::Specifiers::Single(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    let merging_specifiers = vec![
        /* "<spell specifier>" on its own can make a spell specifiers node */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpellSpecifier { specifier: dummy() }.id()]),
            merged: ParserNode::SpellSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpellSpecifier { specifier }] => Ok(ParserNode::SpellSpecifiers {
                    specifiers: object::specified_object::Specifiers::Single(specifier.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<spell specifier> <spell specifier>" -> and list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::SpellSpecifier { specifier: dummy() }.id(),
                ParserNode::SpellSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::SpellSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::SpellSpecifier { specifier: s1 },
                    ParserNode::SpellSpecifier { specifier: s2 },
                ] => Ok(ParserNode::SpellSpecifiers {
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
        /* "<spell specifier> or <spell specifier>" -> or list */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::SpellSpecifier { specifier: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellSpecifier { specifier: dummy() }.id(),
            ]),
            merged: ParserNode::SpellSpecifiers { specifiers: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::SpellSpecifier { specifier: s1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::SpellSpecifier { specifier: s2 },
                ] => Ok(ParserNode::SpellSpecifiers {
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

    [common_specifiers, merging_specifiers].into_iter().flatten()
}
