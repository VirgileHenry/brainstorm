use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate::NumberOperation;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* "<number>/<nmber>" is a power/toughness node */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::Number {
                number: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number {
                number: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::PowerToughness {
            power_toughness: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::Number { number: power },
                ParserNode::LexerToken(Token::NumberOperation(NumberOperation::BarSymbol { .. })),
                ParserNode::Number { number: toughness },
            ] => Ok(ParserNode::PowerToughness {
                power_toughness: boseiju_tree::ability_tree::power_toughness::PowerToughness {
                    power: power.clone(),
                    toughness: toughness.clone(),
                    #[cfg(feature = "spanned_tree")]
                    span: power.span().merge(&toughness.span()),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
