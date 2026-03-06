use crate::ability_tree::time;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "Until end of turn, <continuous effect>" makes a generated continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::ForwardDuration(time::ForwardDuration::UntilEndOfTurn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ContinuousEffect { effect: dummy() }.id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::ForwardDuration(time::ForwardDuration::UntilEndOfTurn {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::ContinuousEffect { effect },
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::GenerateContinuousEffect(
                        crate::ability_tree::imperative::GenerateContinuousEffectImperative {
                            effect: effect.clone(),
                            duration: time::ForwardDuration::UntilEndOfTurn {
                                #[cfg(feature = "spanned_tree")]
                                span: *start_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: effect.node_span().merge(start_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<continuous effect> until end of turn" makes a generated continuous effect. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::ContinuousEffect { effect: dummy() }.id(),
                ParserNode::LexerToken(Token::ForwardDuration(time::ForwardDuration::UntilEndOfTurn {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ContinuousEffect { effect },
                    ParserNode::LexerToken(Token::ForwardDuration(time::ForwardDuration::UntilEndOfTurn {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::GenerateContinuousEffect(
                        crate::ability_tree::imperative::GenerateContinuousEffectImperative {
                            effect: effect.clone(),
                            duration: time::ForwardDuration::UntilEndOfTurn {
                                #[cfg(feature = "spanned_tree")]
                                span: *end_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: effect.node_span().merge(end_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
