use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::ability_tree::time;
use crate::lexer::tokens::Token;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* until end of turn to forward duration */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::ForwardDuration(time::ForwardDuration::UntilEndOfTurn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ForwardDuration { duration: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::ForwardDuration(duration))] => Ok(ParserNode::ForwardDuration {
                    duration: duration.clone(),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* until end of next turn to forward duration */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::ForwardDuration(
                time::ForwardDuration::UntilEndOfYourNextTurn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::ForwardDuration { duration: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(Token::ForwardDuration(duration))] => Ok(ParserNode::ForwardDuration {
                    duration: duration.clone(),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
