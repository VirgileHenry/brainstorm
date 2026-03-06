use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Player may ability, without consequences */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList { imperatives: dummy() }.id(),
            ]),
            merged: ParserNode::Statement { statement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May { .. })),
                    ParserNode::ImperativeList { imperatives }, // Fixme: there are also "if they don't / if they do" stuff
                ] => Ok(ParserNode::Statement {
                    statement: crate::ability_tree::statement::Statement::May(crate::ability_tree::statement::MayAbility {
                        player: player.clone(),
                        action: imperatives.clone(),
                        if_it_is_done: None,
                        if_not_done: None,
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(&imperatives.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* May ability with an "if you do" consequence */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList { imperatives: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Do {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Statement { statement: dummy() }.id(),
            ]),
            merged: ParserNode::Statement { statement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May { .. })),
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If { .. })),
                    ParserNode::LexerToken(_), /* Only for matching the proper rule tokens */
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Do { .. })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::Statement {
                    statement: crate::ability_tree::statement::Statement::May(crate::ability_tree::statement::MayAbility {
                        player: player.clone(),
                        action: imperatives.clone(),
                        if_it_is_done: Some(Box::new(statement.clone())),
                        if_not_done: None,
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(&statement.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* May ability with an "if they do" consequence */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList { imperatives: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::They {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Do {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Statement { statement: dummy() }.id(),
            ]),
            merged: ParserNode::Statement { statement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May { .. })),
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If { .. })),
                    ParserNode::LexerToken(_), /* Only for matching the proper rule tokens */
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Do { .. })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::Statement {
                    statement: crate::ability_tree::statement::Statement::May(crate::ability_tree::statement::MayAbility {
                        player: player.clone(),
                        action: imperatives.clone(),
                        if_it_is_done: Some(Box::new(statement.clone())),
                        if_not_done: None,
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(&statement.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* May ability with an "if you don't" consequence */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList { imperatives: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::PlayerSpecifier(intermediates::PlayerSpecifier::You {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Dont {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Statement { statement: dummy() }.id(),
            ]),
            merged: ParserNode::Statement { statement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May { .. })),
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If { .. })),
                    ParserNode::LexerToken(_), /* Only for matching the proper rule tokens */
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Dont { .. })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::Statement {
                    statement: crate::ability_tree::statement::Statement::May(crate::ability_tree::statement::MayAbility {
                        player: player.clone(),
                        action: imperatives.clone(),
                        if_it_is_done: None,
                        if_not_done: Some(Box::new(statement.clone())),
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(&statement.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* May ability with an "if they don't" consequence */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Player { player: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ImperativeList { imperatives: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::They {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Dont {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Statement { statement: dummy() }.id(),
            ]),
            merged: ParserNode::Statement { statement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Player { player },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::May { .. })),
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If { .. })),
                    ParserNode::LexerToken(_), /* Only for matching the proper rule tokens */
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Dont { .. })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::Statement { statement },
                ] => Ok(ParserNode::Statement {
                    statement: crate::ability_tree::statement::Statement::May(crate::ability_tree::statement::MayAbility {
                        player: player.clone(),
                        action: imperatives.clone(),
                        if_it_is_done: None,
                        if_not_done: Some(Box::new(statement.clone())),
                        #[cfg(feature = "spanned_tree")]
                        span: player.node_span().merge(&statement.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* An imperative list can make a statement. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::ImperativeList { imperatives: dummy() }.id()]),
            merged: ParserNode::Statement { statement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ImperativeList { imperatives }] => Ok(ParserNode::Statement {
                    statement: crate::ability_tree::statement::Statement::Imperatives(imperatives.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
