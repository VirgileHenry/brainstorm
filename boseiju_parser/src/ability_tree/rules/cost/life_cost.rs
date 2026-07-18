use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    std::iter::once(/* "pay <number> life" */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                token: intermediate::PlayerAction::Pay {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
                tense: boseiju_lexer::Tense::BaseForm,
            }))
            .id(),
            ParserNode::Number {
                number: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Life {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::ImperativeAsCost {
            cost: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::TensedPlayerAction(intermediate::TensedPlayerAction {
                    token:
                        intermediate::PlayerAction::Pay {
                            #[cfg(feature = "spanned_tree")]
                                span: pay_span,
                        },
                    tense: boseiju_lexer::Tense::BaseForm,
                })),
                ParserNode::Number { number },
                ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Life {
                    #[cfg(feature = "spanned_tree")]
                        span: life_span,
                })),
            ] => Ok(ParserNode::ImperativeAsCost {
                cost: boseiju_tree::ability_tree::imperative::Imperative {
                    kind: boseiju_tree::ability_tree::imperative::ImperativeKind::PayLife(
                        boseiju_tree::ability_tree::imperative::PayLifeImperative {
                            amount: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: pay_span.merge(life_span),
                        },
                    ),
                    executing_player: boseiju_tree::ability_tree::player::PlayerSpecifier::You {
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
