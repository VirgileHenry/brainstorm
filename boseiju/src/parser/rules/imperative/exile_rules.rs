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
        /* Exile object reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Exile {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Exile {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::ObjectReference { reference },
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::Exile(
                        crate::ability_tree::imperative::ExileImperative {
                            object: reference.clone(),
                            follow_up: None,
                            #[cfg(feature = "spanned_tree")]
                            span: span.merge(&reference.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Exile object reference, with a follow up on the exile thing: "then return it..." */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Exile {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ExileFollowUp { follow_up: dummy() }.id(),
            ]),
            merged: ParserNode::Imperative { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Exile {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot { .. })),
                    ParserNode::ExileFollowUp { follow_up },
                ] => Ok(ParserNode::Imperative {
                    imperative: crate::ability_tree::imperative::Imperative::Exile(
                        crate::ability_tree::imperative::ExileImperative {
                            object: reference.clone(),
                            follow_up: Some(follow_up.clone()),
                            #[cfg(feature = "spanned_tree")]
                            span: span.merge(&follow_up.node_span()),
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
