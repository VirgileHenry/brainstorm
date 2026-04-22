use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* where X is the number of <object reference> on the battlefield */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Where {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediates::Number::X {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Is {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::The {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::Number(intermediates::Number::NumberOf {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ObjectReference { reference: dummy() }.id(),
                /* Fixme: can "on the battlefield" be part of the object specifier ? */
                /* Maybe "InZone" is a sane specifier */
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::GlobalZone(intermediates::GlobalZone::TheBattlefield {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::XDefinition { definition: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Where {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::X { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Is { .. })),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::The { .. })),
                    ParserNode::LexerToken(Token::Number(intermediates::Number::NumberOf { .. })),
                    ParserNode::ObjectReference { reference },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::On {
                        #[cfg(feature = "spanned_tree")]
                            span: battlefield_start_span,
                    })),
                    ParserNode::LexerToken(Token::GlobalZone(intermediates::GlobalZone::TheBattlefield {
                        #[cfg(feature = "spanned_tree")]
                            span: battlefield_end_span,
                    })),
                ] => Ok(ParserNode::XDefinition {
                    definition: crate::ability_tree::number::XDefinition::NumberOfObjects(
                        crate::ability_tree::number::XNumberOfObjects {
                            object: reference.clone(),
                            in_zone: crate::ability_tree::zone::ZoneReference::TheBattlefield {
                                #[cfg(feature = "spanned_tree")]
                                span: battlefield_start_span.merge(battlefield_end_span),
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: start_span.merge(battlefield_end_span),
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
