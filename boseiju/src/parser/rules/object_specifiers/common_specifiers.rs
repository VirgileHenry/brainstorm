use crate::ability_tree::terminals;
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

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "<color>" makes a color specifier */
    let color_to_specifier = terminals::Color::all()
        .map(|color| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Color(color.clone())).id()]),
            merged: ParserNode::ColorSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::Color(color))] => Ok(ParserNode::ColorSpecifier {
                    specifier: crate::ability_tree::object::specified_object::ColorSpecifier {
                        color: color.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: color.span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let control_specifier = vec![
        /* "<player> controls" is a control specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Control {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ControlSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Control {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::ControlSpecifier {
                    specifier: crate::ability_tree::object::specified_object::ControlSpecifier {
                        controller: player.clone(),
                        controlled: true,
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<player> don't controls" is a control specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Dont {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Control {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ControlSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Dont { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Control {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::ControlSpecifier {
                    specifier: crate::ability_tree::object::specified_object::ControlSpecifier {
                        controller: player.clone(),
                        controlled: false,
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "others" is a another specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Other {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::AnotherSpecifier { specifier: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Other {
                        #[cfg(feature = "spanned_tree")]
                            span: another_span,
                    })),
                ] => Ok(ParserNode::AnotherSpecifier {
                    specifier: crate::ability_tree::object::specified_object::AnotherObjectSpecifier {
                        #[cfg(feature = "spanned_tree")]
                        span: *another_span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [color_to_specifier, control_specifier].into_iter().flatten()
}
