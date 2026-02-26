use crate::ability_tree::object;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Create token with no specifiers */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::EventSource { source: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Supertype(
                    mtg_data::Supertype::Token,
                )))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Supertype(mtg_data::Supertype::Token))),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreateTokens(crate::ability_tree::event::CreateTokensEvent {
                        source: source.clone(),
                        quantity: number.clone(),
                        token_specifiers: None,
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
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Supertype(
                    mtg_data::Supertype::Token,
                )))
                .id(),
                ParserNode::LexerToken(TokenKind::UnderControl(non_terminals::UnderControl::UnderYourControl)).id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Supertype(mtg_data::Supertype::Token))),
                    ParserNode::LexerToken(TokenKind::UnderControl(non_terminals::UnderControl::UnderYourControl)),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreateTokens(crate::ability_tree::event::CreateTokensEvent {
                        source: source.clone(),
                        quantity: number.clone(),
                        token_specifiers: Some(crate::ability_tree::object::ObjectSpecifiers::Single(
                            crate::ability_tree::object::ObjectSpecifier::Control(terminals::ControlSpecifier::YouControl),
                        )),
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
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)).id(),
                ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)).id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Supertype(
                    mtg_data::Supertype::Token,
                )))
                .id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::EventSource { source },
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Would)),
                    ParserNode::LexerToken(TokenKind::KeywordAction(mtg_data::KeywordAction::Create)),
                    ParserNode::Number { number },
                    ParserNode::ObjectSpecifiers { specifiers },
                    ParserNode::LexerToken(TokenKind::ObjectKind(object::ObjectKind::Supertype(mtg_data::Supertype::Token))),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::CreateTokens(crate::ability_tree::event::CreateTokensEvent {
                        source: source.clone(),
                        quantity: number.clone(),
                        token_specifiers: Some(specifiers.clone()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
