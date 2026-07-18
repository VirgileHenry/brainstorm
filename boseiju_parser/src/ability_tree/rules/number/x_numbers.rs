use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::number;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* X on its own means that the x had to be in the cost */
        /* Fixme: maybe context could help to ensure that's the case ? */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Number {
                number: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Number(intermediate::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: x_span,
                    })),
                ] => Ok(ParserNode::Number {
                    number: number::Number::X(number::XNumber {
                        x_definition: Box::new(boseiju_tree::ability_tree::number::XDefinition::FromCost(
                            boseiju_tree::ability_tree::number::XFromCost {
                                #[cfg(feature = "spanned_tree")]
                                span: x_span.clone(),
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: x_span.clone(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
