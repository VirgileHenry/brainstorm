#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "Draw <number> cards" makes a draw card imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Draw {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Number { number: dummy() }.id(),
                ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Card(crate::utils::dummy()))).id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Draw {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Number { number },
                    ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Card(_card))),
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::Draw(
                        crate::ability_tree::imperative::DrawImperative {
                            amount: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: start_span.merge(&_card.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "Draw a card for each <object reference>" makes a draw X card imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Draw {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Card(crate::utils::dummy()))).id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ForEach {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::PlayerAction(intermediates::PlayerAction::Draw {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::A { .. })),
                    ParserNode::LexerToken(Token::ObjectKind(object::ObjectKind::Card(_card))),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ForEach {
                        #[cfg(feature = "spanned_tree")]
                            span: for_each_span,
                    })),
                    ParserNode::ObjectReference { reference },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::Draw(
                        crate::ability_tree::imperative::DrawImperative {
                            amount: crate::ability_tree::number::Number::X(crate::ability_tree::number::XNumber {
                                x_definition: Box::new(crate::ability_tree::number::XDefinition::NumberOfObjects(
                                    crate::ability_tree::number::XNumberOfObjects {
                                        object: reference.clone(),
                                        in_zone: crate::ability_tree::zone::ZoneReference::TheBattlefield {
                                            #[cfg(feature = "spanned_tree")]
                                            span: reference.node_span().empty_at_end(),
                                        },
                                        #[cfg(feature = "spanned_tree")]
                                        span: reference.node_span().merge(for_each_span),
                                    },
                                )),
                                #[cfg(feature = "spanned_tree")]
                                span: reference.node_span().merge(for_each_span),
                            }),
                            #[cfg(feature = "spanned_tree")]
                            span: reference.node_span().merge(start_span),
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
