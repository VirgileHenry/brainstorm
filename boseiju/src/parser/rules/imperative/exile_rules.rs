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
        /* "exile <permanent reference>" -> move object from battlfield to exile */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Exile {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Permanent { permanent: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Exile {
                        #[cfg(feature = "spanned_tree")]
                            span: exile_span,
                    })),
                    ParserNode::Permanent { permanent },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::ChangeZone(
                        crate::ability_tree::imperative::ChangeZoneImperative {
                            object: permanent.to_card(),
                            from: crate::ability_tree::zone::ZoneReference::TheBattlefield {
                                #[cfg(feature = "spanned_tree")]
                                span: permanent.node_span().empty_at_end(),
                            },
                            to: crate::ability_tree::zone::ZoneReference::Exile {
                                #[cfg(feature = "spanned_tree")]
                                span: *exile_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: permanent.node_span().merge(exile_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "exile <card reference> from <zone>" means to move from <zone> to <exile> */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Exile {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Card { card: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::ZoneReference { zone: dummy() }.id(),
            ]),
            merged: ParserNode::ImperativeKind { imperative: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Exile {
                        #[cfg(feature = "spanned_tree")]
                            span: exile_span,
                    })),
                    ParserNode::Card { card },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::From { .. })),
                    ParserNode::ZoneReference { zone },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: crate::ability_tree::imperative::ImperativeKind::ChangeZone(
                        crate::ability_tree::imperative::ChangeZoneImperative {
                            object: card.clone(),
                            from: zone.clone(),
                            to: crate::ability_tree::zone::ZoneReference::Exile {
                                #[cfg(feature = "spanned_tree")]
                                span: *exile_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: zone.node_span().merge(exile_span),
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
