use super::ParserNode;
use crate::ability_tree::time;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let backward_duration_to_event_occured_condition = [time::BackwardDuration::ThisTurn]
        .into_iter()
        .map(|duration| super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)).id(),
                ParserNode::Event { event: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::BackwardDuration(duration)).id(),
            ]),
            merged: ParserNode::IfCondition { condition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)),
                    ParserNode::Event { event },
                    ParserNode::LexerToken(TokenKind::BackwardDuration(duration)),
                ] => Ok(ParserNode::IfCondition {
                    condition: crate::ability_tree::if_condition::IfCondition::EventOccured(
                        crate::ability_tree::if_condition::IfConditionEventOccured {
                            timeframe: *duration,
                            event: event.clone(),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    [backward_duration_to_event_occured_condition].into_iter().flatten()
}
