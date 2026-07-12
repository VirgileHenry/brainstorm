use crate::ability_tree::number;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* X on its own means that the x had to be in the cost */
        /* Fixme: maybe context could help to ensure that's the case ? */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::Number { number: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                        #[cfg(feature = "spanned_tree")]
                            span: x_span,
                    })),
                ] => Ok(ParserNode::Number {
                    number: number::Number::X(number::XNumber {
                        x_definition: Box::new(crate::ability_tree::number::XDefinition::FromCost(
                            crate::ability_tree::number::XFromCost {
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
