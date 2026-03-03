use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let may_abilities_from_players = [
        (
            Token::PlayerSpecifier(terminals::PlayerSpecifier::Any {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            Token::EnglishKeyword(intermediates::EnglishKeyword::They {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
        ),
        (
            Token::PlayerSpecifier(terminals::PlayerSpecifier::ToYourLeft {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            Token::EnglishKeyword(intermediates::EnglishKeyword::They {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
        ),
        (
            Token::PlayerSpecifier(terminals::PlayerSpecifier::ToYourRight {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            Token::EnglishKeyword(intermediates::EnglishKeyword::They {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
        ),
        (
            Token::PlayerSpecifier(terminals::PlayerSpecifier::ToYourRight {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            Token::EnglishKeyword(intermediates::EnglishKeyword::They {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
        ),
        (
            Token::PlayerSpecifier(terminals::PlayerSpecifier::You {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
            Token::PlayerSpecifier(terminals::PlayerSpecifier::You {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }),
        ),
    ]
    .into_iter()
    .map(|(player, later_player_ref)| {
        [
            /* Player may ability, without consequences */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(player.clone()).id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::Imperative { imperative: dummy() }.id(),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                        span: Default::default(),
                    }))
                    .id(),
                ]),
                merged: ParserNode::Statement { statement: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::PlayerSpecifier(player)),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May { .. })),
                        ParserNode::Imperative { imperative }, // Fixme: there are also "if they don't / if they do" stuff
                        ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { span: dot_span })),
                    ] => Ok(ParserNode::Statement {
                        statement: crate::ability_tree::statement::Statement::May(crate::ability_tree::statement::MayAbility {
                            player: *player,
                            action: imperative.clone(),
                            if_it_is_done: None,
                            if_not_done: None,
                            span: player.span().merge(dot_span),
                        }),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* May ability with an "if they do" consequence */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(player.clone()).id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::Imperative { imperative: dummy() }.id(),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(later_player_ref.clone()).id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Do {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::Statement { statement: dummy() }.id(),
                ]),
                merged: ParserNode::Statement { statement: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::PlayerSpecifier(player)),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May { .. })),
                        ParserNode::Imperative { imperative },
                        ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { .. })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If { .. })),
                        ParserNode::LexerToken(_), /* Only for matching the proper rule tokens */
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Do { .. })),
                        ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                        ParserNode::Statement { statement },
                    ] => Ok(ParserNode::Statement {
                        statement: crate::ability_tree::statement::Statement::May(crate::ability_tree::statement::MayAbility {
                            player: *player,
                            action: imperative.clone(),
                            if_it_is_done: Some(Box::new(statement.clone())),
                            if_not_done: None,
                            span: player.span().merge(&statement.span()),
                        }),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* May ability with an "if they don't" consequence */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(player.clone()).id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::Imperative { imperative: dummy() }.id(),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(later_player_ref.clone()).id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Dont {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::Statement { statement: dummy() }.id(),
                ]),
                merged: ParserNode::Statement { statement: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::PlayerSpecifier(player)),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May { .. })),
                        ParserNode::Imperative { imperative },
                        ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { .. })),
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If { .. })),
                        ParserNode::LexerToken(_), /* Only for matching the proper rule tokens */
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Dont { .. })),
                        ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                        ParserNode::Statement { statement },
                    ] => Ok(ParserNode::Statement {
                        statement: crate::ability_tree::statement::Statement::May(crate::ability_tree::statement::MayAbility {
                            player: *player,
                            action: imperative.clone(),
                            if_it_is_done: None,
                            if_not_done: Some(Box::new(statement.clone())),
                            span: player.span().merge(&statement.span()),
                        }),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
        ]
    })
    .flatten()
    .collect::<Vec<_>>();

    let default_statement_rules = vec![
        /* An imperative list with its closing dot can make a statement. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::ImperativeList { imperatives: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Statement { statement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { span: dot_span })),
                ] => Ok(ParserNode::Statement {
                    statement: crate::ability_tree::statement::Statement::Imperatives({
                        let mut imperatives = imperatives.clone();
                        imperatives.span = imperatives.span.merge(dot_span);
                        imperatives
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [may_abilities_from_players, default_statement_rules].into_iter().flatten()
}
