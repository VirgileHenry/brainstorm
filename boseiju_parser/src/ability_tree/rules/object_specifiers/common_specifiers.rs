use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_lexer::terminal;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* "<color>" makes a color specifier */
    let color_to_specifier = terminal::Color::all()
        .map(|color| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Color(color.clone())).id()]),
            merged: ParserNode::ColorSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::Color(color))] => Ok(ParserNode::ColorSpecifier {
                    specifier: boseiju_tree::ability_tree::object::specified_object::ColorSpecifier {
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
                ParserNode::Player {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Control {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ControlSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Control {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::ControlSpecifier {
                    specifier: boseiju_tree::ability_tree::object::specified_object::ControlSpecifier {
                        controller: player.clone(),
                        controlled: true,
                        #[cfg(feature = "spanned_tree")]
                        span: player.span().merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<player> don't controls" is a control specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Player {
                    player: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Dont {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Control {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ControlSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Dont { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::Control {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::ControlSpecifier {
                    specifier: boseiju_tree::ability_tree::object::specified_object::ControlSpecifier {
                        controller: player.clone(),
                        controlled: false,
                        #[cfg(feature = "spanned_tree")]
                        span: player.span().merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "others" is a another specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Other {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::AnotherSpecifier {
                specifier: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Other {
                        #[cfg(feature = "spanned_tree")]
                            span: another_span,
                    })),
                ] => Ok(ParserNode::AnotherSpecifier {
                    specifier: boseiju_tree::ability_tree::object::specified_object::AnotherObjectSpecifier {
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
