use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [boseiju_lexer::terminal::BackwardDuration::ThisTurn {
        #[cfg(feature = "spanned_tree")]
        span: Default::default(),
    }]
    .into_iter()
    .map(|duration| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::Event {
                event: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::BackwardDuration(duration)).id(),
        ]),
        merged: ParserNode::Condition {
            condition: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::Event { event },
                ParserNode::LexerToken(Token::BackwardDuration(duration)),
            ] => Ok(ParserNode::Condition {
                condition: boseiju_tree::ability_tree::conditional::Condition::EventOccured(
                    boseiju_tree::ability_tree::conditional::ConditionEventOccured {
                        timeframe: *duration,
                        event: event.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: event.span().merge(&duration.span()),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
