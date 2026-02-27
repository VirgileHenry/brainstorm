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
        /* Object enters the battlefield event */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::CardActions(non_terminals::CardActions::Enters)).id(),
                ParserNode::LexerToken(TokenKind::GlobalZone(non_terminals::GlobalZone::TheBattlefield)).id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::CardActions(non_terminals::CardActions::Enters)),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::EntersTheBattlefield(
                        crate::ability_tree::event::EntersTheBattlefieldEvent {
                            object: reference.clone(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Since foundation, "enters" is a shortcut for "enters the battlefield" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::CardActions(non_terminals::CardActions::Enters)).id(),
            ]),
            merged: ParserNode::Event { event: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(TokenKind::CardActions(non_terminals::CardActions::Enters)),
                ] => Ok(ParserNode::Event {
                    event: crate::ability_tree::event::Event::EntersTheBattlefield(
                        crate::ability_tree::event::EntersTheBattlefieldEvent {
                            object: reference.clone(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
