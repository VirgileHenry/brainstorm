use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    std::iter::once(/* "pay <number> life" */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Pay {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Life {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::ImperativeAsCost { cost: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Pay {
                    #[cfg(feature = "spanned_tree")]
                        span: pay_span,
                })),
                ParserNode::Number { number },
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Life {
                    #[cfg(feature = "spanned_tree")]
                        span: life_span,
                })),
            ] => Ok(ParserNode::ImperativeAsCost {
                cost: crate::ability_tree::imperative::Imperative {
                    kind: crate::ability_tree::imperative::ImperativeKind::PayLife(
                        crate::ability_tree::imperative::PayLifeImperative {
                            amount: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: pay_span.merge(life_span),
                        },
                    ),
                    executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                        #[cfg(feature = "spanned_tree")]
                        span: pay_span.empty_at_start(),
                    },
                    #[cfg(feature = "spanned_tree")]
                    span: pay_span.merge(life_span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
