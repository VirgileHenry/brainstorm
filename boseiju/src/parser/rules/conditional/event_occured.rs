use crate::ability_tree::time;
use crate::lexer::tokens::Token;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [time::BackwardDuration::ThisTurn {
        #[cfg(feature = "spanned_tree")]
        span: Default::default(),
    }]
    .into_iter()
    .map(|duration| ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::Event { event: dummy() }.id(),
            ParserNode::LexerToken(Token::BackwardDuration(duration)).id(),
        ]),
        merged: ParserNode::Condition { condition: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::Event { event },
                ParserNode::LexerToken(Token::BackwardDuration(duration)),
            ] => Ok(ParserNode::Condition {
                condition: crate::ability_tree::conditional::Condition::EventOccured(
                    crate::ability_tree::conditional::ConditionEventOccured {
                        timeframe: *duration,
                        event: event.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: event.node_span().merge(&duration.node_span()),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
