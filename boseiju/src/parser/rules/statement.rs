use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let may_abilities_from_players = [
        (
            TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::Any),
            TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::They),
        ),
        (
            TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::ToYourLeft),
            TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::They),
        ),
        (
            TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::ToYourRight),
            TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::They),
        ),
        (
            TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::ToYourRight),
            TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::They),
        ),
        (
            TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::You),
            TokenKind::PlayerSpecifier(terminals::PlayerSpecifier::You),
        ),
    ]
    .into_iter()
    .map(|(player, later_player_ref)| {
        [
            /* Player may ability, without consequences */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(player).id(),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)).id(),
                    ParserNode::Imperative { imperative: dummy() }.id(),
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
                ]),
                merged: ParserNode::Statement { statement: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)),
                        ParserNode::Imperative { imperative }, // Fixme: there are also "if they don't / if they do" stuff
                        ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
                    ] => Ok(ParserNode::Statement {
                        statement: crate::ability_tree::statement::Statement::May(crate::ability_tree::statement::MayAbility {
                            player: *player,
                            action: imperative.clone(),
                            if_it_is_done: None,
                            if_not_done: None,
                        }),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* May ability with an "if they do" consequence */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(player).id(),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)).id(),
                    ParserNode::Imperative { imperative: dummy() }.id(),
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)).id(),
                    ParserNode::LexerToken(later_player_ref).id(),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Do)).id(),
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                    ParserNode::Statement { statement: dummy() }.id(),
                ]),
                merged: ParserNode::Statement { statement: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)),
                        ParserNode::Imperative { imperative },
                        ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)),
                        ParserNode::LexerToken(_), /* Only for matching the proper rule tokens */
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Do)),
                        ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                        ParserNode::Statement { statement },
                    ] => Ok(ParserNode::Statement {
                        statement: crate::ability_tree::statement::Statement::May(crate::ability_tree::statement::MayAbility {
                            player: *player,
                            action: imperative.clone(),
                            if_it_is_done: Some(Box::new(statement.clone())),
                            if_not_done: None,
                        }),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* May ability with an "if they don't" consequence */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(player).id(),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)).id(),
                    ParserNode::Imperative { imperative: dummy() }.id(),
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)).id(),
                    ParserNode::LexerToken(later_player_ref).id(),
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Dont)).id(),
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
                    ParserNode::Statement { statement: dummy() }.id(),
                ]),
                merged: ParserNode::Statement { statement: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)),
                        ParserNode::Imperative { imperative },
                        ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)),
                        ParserNode::LexerToken(_), /* Only for matching the proper rule tokens */
                        ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Dont)),
                        ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                        ParserNode::Statement { statement },
                    ] => Ok(ParserNode::Statement {
                        statement: crate::ability_tree::statement::Statement::May(crate::ability_tree::statement::MayAbility {
                            player: *player,
                            action: imperative.clone(),
                            if_it_is_done: None,
                            if_not_done: Some(Box::new(statement.clone())),
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
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
            ]),
            merged: ParserNode::Statement { statement: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ImperativeList { imperatives },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
                ] => Ok(ParserNode::Statement {
                    statement: crate::ability_tree::statement::Statement::Imperatives(imperatives.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [may_abilities_from_players, default_statement_rules].into_iter().flatten()
}
