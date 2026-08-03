use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use boseiju_lexer::Token;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* until end of turn to forward duration */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::ForwardDuration(
                boseiju_lexer::terminal::ForwardDuration::UntilEndOfTurn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::ForwardDuration {
                duration: Default::default(),
            }
            .id(),
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
                boseiju_lexer::terminal::ForwardDuration::UntilEndOfYourNextTurn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::ForwardDuration {
                duration: Default::default(),
            }
            .id(),
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
