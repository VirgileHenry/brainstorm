use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Create token with no specifiers */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Would {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Create,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Supertype(object::Supertype {
                    supertype: mtg_data::Supertype::Token,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                })))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Would { .. })),
                    ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Create,
                        ..
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Supertype(_supertype))),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreateTokens(crate::ability_tree::event::CreateTokensEvent {
                        source: source.clone(),
                        quantity: number.clone(),
                        token_specifiers: None,
                        #[cfg(feature = "spanned_tree")]
                        span: source.node_span().merge(&_supertype.span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Create token with special "under your control" specifier */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Would {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Create,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Supertype(object::Supertype {
                    supertype: mtg_data::Supertype::Token,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                })))
                .id(),
                ParserNode::LexerToken(Token::UnderControl(intermediates::UnderControl::UnderYourControl {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Would { .. })),
                    ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Create,
                        ..
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Supertype(object::Supertype {
                        supertype: mtg_data::Supertype::Token,
                        ..
                    }))),
                    ParserNode::LexerToken(Token::UnderControl(intermediates::UnderControl::UnderYourControl {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreateTokens(crate::ability_tree::event::CreateTokensEvent {
                        source: source.clone(),
                        quantity: number.clone(),
                        token_specifiers: Some(crate::ability_tree::object::ObjectSpecifiers::Single(
                            crate::ability_tree::object::ObjectSpecifier::Control(object::ControlSpecifier {
                                controller: crate::ability_tree::player::PlayerSpecifier::You {
                                    #[cfg(feature = "spanned_tree")]
                                    span: *span,
                                },
                                controlled: true,
                                #[cfg(feature = "spanned_tree")]
                                span: *span,
                            }),
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: source.node_span().merge(span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Create token with specifiers before "token", as in "if you would create a treasure token" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Would {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                    keyword_action: mtg_data::KeywordAction::Create,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
                ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Supertype(object::Supertype {
                    supertype: mtg_data::Supertype::Token,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                })))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Would { .. })),
                    ParserNode::LexerToken(Token::KeywordAction(intermediates::KeywordAction {
                        keyword_action: mtg_data::KeywordAction::Create,
                        ..
                    })),
                    ParserNode::Number { number },
                    ParserNode::ObjectSpecifiers { specifiers },
                    ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Supertype(_supertype))),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreateTokens(crate::ability_tree::event::CreateTokensEvent {
                        source: source.clone(),
                        quantity: number.clone(),
                        token_specifiers: Some(specifiers.clone()),
                        #[cfg(feature = "spanned_tree")]
                        span: source.node_span().merge(&_supertype.span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
