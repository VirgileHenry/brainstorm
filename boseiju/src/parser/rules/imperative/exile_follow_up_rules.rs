use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::time;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [/* Exile follow up that return the card */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Return {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::PreviouslyMentionnedObject { object: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::ZoneReference { zone: dummy() }.id(),
            ParserNode::LexerToken(Token::UnderControl(intermediates::UnderControl::UnderItsOwnersControl {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::At {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::Instant(time::Instant::TheBeginningOfTheNextEndStep {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::ExileFollowUp { follow_up: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Return {
                    #[cfg(feature = "spanned_tree")]
                        span: start_span,
                })),
                ParserNode::PreviouslyMentionnedObject { object },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::To { .. })),
                ParserNode::ZoneReference { zone },
                ParserNode::LexerToken(Token::UnderControl(intermediates::UnderControl::UnderItsOwnersControl { .. })),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::At { .. })),
                ParserNode::LexerToken(Token::Instant(time::Instant::TheBeginningOfTheNextEndStep {
                    #[cfg(feature = "spanned_tree")]
                        span: end_span,
                })),
            ] => Ok(ParserNode::ExileFollowUp {
                follow_up: crate::ability_tree::imperative::ExileFollowUp::ReturnIt(
                    crate::ability_tree::imperative::ExileFollowUpReturn {
                        return_imperative: crate::ability_tree::imperative::ReturnImperative {
                            object: crate::ability_tree::object::ObjectReference::PreviouslyMentionned(object.clone()),
                            from: crate::ability_tree::zone::ZoneReference::Exile {
                                #[cfg(feature = "spanned_tree")]
                                span: start_span.empty_at_end(),
                            },
                            to: zone.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: start_span.merge(&zone.node_span()),
                        },
                        at: Some(time::Instant::TheBeginningOfTheNextEndStep { span: *end_span }),
                        #[cfg(feature = "spanned_tree")]
                        span: start_span.merge(end_span),
                    },
                ),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
