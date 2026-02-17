use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
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
        from: super::RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)).id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)).id(),
            ParserNode::Imperative {
                imperative: DummyInit::dummy_init(),
            }
            .id(),
        ]),
        result: ParserNode::Statement {
            statement: DummyInit::dummy_init(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::PlayerSpecifier(player)),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::May)),
                ParserNode::Imperative { imperative }, // Fixme: there are also "if they don't / if they do" stuff
            ] => Some(ParserNode::Statement {
                statement: crate::ability_tree::statement::Statement::May {
                    player: *player,
                    action: imperative.clone(),
                },
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let default_statement_rules = vec![super::ParserRule {
        from: super::RuleLhs::new(&[ParserNode::Imperative {
            imperative: DummyInit::dummy_init(),
        }
        .id()]),
        result: ParserNode::Statement {
            statement: DummyInit::dummy_init(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::Imperative { imperative }] => Some(ParserNode::Statement {
                statement: crate::ability_tree::statement::Statement::Imperative(imperative.clone()),
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    }];

    [may_abilities_from_players, default_statement_rules].into_iter().flatten()
}
