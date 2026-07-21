use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "return <permanent reference> to <zone>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Return {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Permanent {
                    permanent: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference {
                    zone: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Return {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Permanent { permanent },
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To { .. })),
                    ParserNode::ZoneReference { zone: to_zone },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::ChangeZone(
                        boseiju_tree::ability_tree::imperative::ChangeZoneImperative {
                            object: permanent.to_card(),
                            from: boseiju_tree::ability_tree::zone::ZoneReference::TheBattlefield {
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.span().empty_at_end(),
                            },
                            to: to_zone.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: start_span.merge(&to_zone.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "return <card reference> from <zone> to <zone>" */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Return {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Card {
                    card: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::From {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference {
                    zone: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference {
                    zone: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediate::AmbiguousToken::Return {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::Card { card },
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::From { .. })),
                    ParserNode::ZoneReference { zone: from_zone },
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::To { .. })),
                    ParserNode::ZoneReference { zone: to_zone },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::ChangeZone(
                        boseiju_tree::ability_tree::imperative::ChangeZoneImperative {
                            object: card.clone(),
                            from: from_zone.clone(),
                            to: to_zone.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: start_span.merge(&to_zone.span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "put <card reference> from <zone> onto <zone> */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::TensedActionKeyword(intermediate::TensedActionKeyword {
                    token: intermediate::ActionKeyword::Put {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    },
                    tense: boseiju_lexer::Tense::BaseForm,
                }))
                .id(),
                ParserNode::Card {
                    card: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::From {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference {
                    zone: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::On {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference {
                    zone: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::TensedActionKeyword(intermediate::TensedActionKeyword {
                        token:
                            intermediate::ActionKeyword::Put {
                                #[cfg(feature = "spanned_tree")]
                                    span: start_span,
                            },
                        tense: boseiju_lexer::Tense::BaseForm,
                    })),
                    ParserNode::Card { card },
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::From { .. })),
                    ParserNode::ZoneReference { zone: from_zone },
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::On { .. })),
                    ParserNode::ZoneReference { zone: to_zone },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::ChangeZone(
                        boseiju_tree::ability_tree::imperative::ChangeZoneImperative {
                            object: card.clone(),
                            from: from_zone.clone(),
                            to: to_zone.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: start_span.merge(&to_zone.span()),
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
