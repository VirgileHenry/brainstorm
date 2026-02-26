use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let may_abilities_from_players = [
        terminals::PlayerSpecifier::AnOpponent,
        terminals::PlayerSpecifier::Any,
        terminals::PlayerSpecifier::ToYourLeft,
        terminals::PlayerSpecifier::ToYourRight,
        terminals::PlayerSpecifier::You,
    ]
    .into_iter()
    .map(|player| super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)).id(),
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
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
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
