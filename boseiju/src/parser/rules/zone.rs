use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let default_rules = vec![
        /* The battlefield is a zone reference */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::LexerToken(Token::GlobalZone(
                intermediates::GlobalZone::TheBattlefield {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
            ))
            .id()]),
            merged: ParserNode::ZoneReference { zone: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::GlobalZone(intermediates::GlobalZone::TheBattlefield {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::ZoneReference {
                    zone: crate::ability_tree::zone::ZoneReference::TheBattlefield {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* The battlefield under someone controls can be seen as a owned zone  */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::GlobalZone(intermediates::GlobalZone::TheBattlefield {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::UnderControl(intermediates::UnderControl::UnderYourControl {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::ZoneReference { zone: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::GlobalZone(intermediates::GlobalZone::TheBattlefield {
                        #[cfg(feature = "spanned_tree")]
                            span: the_battlefield_span,
                    })),
                    ParserNode::LexerToken(Token::UnderControl(intermediates::UnderControl::UnderYourControl {
                        #[cfg(feature = "spanned_tree")]
                            span: under_control_span,
                    })),
                ] => Ok(ParserNode::ZoneReference {
                    zone: crate::ability_tree::zone::ZoneReference::OwnedZone(crate::ability_tree::zone::OwnedZone {
                        zone: crate::ability_tree::zone::OwnableZone::Battlefield {
                            #[cfg(feature = "spanned_tree")]
                            span: *the_battlefield_span,
                        },
                        owner: crate::ability_tree::player::PlayerSpecifier::You {
                            #[cfg(feature = "spanned_tree")]
                            span: *under_control_span,
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: the_battlefield_span.merge(under_control_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    let owned_zone_rules = [
        crate::ability_tree::zone::OwnableZone::Graveyard {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        crate::ability_tree::zone::OwnableZone::Hand {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        crate::ability_tree::zone::OwnableZone::Library {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|zone| {
        [
            /* For zones, ambiguous token "your" is valid */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Your {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::OwnableZone(zone)).id(),
                ]),
                merged: ParserNode::ZoneReference { zone: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(Token::AmbiguousToken(intermediates::AmbiguousToken::Your {
                            #[cfg(feature = "spanned_tree")]
                                span: owner_span,
                        })),
                        ParserNode::LexerToken(Token::OwnableZone(zone)),
                    ] => Ok(ParserNode::ZoneReference {
                        zone: crate::ability_tree::zone::ZoneReference::OwnedZone(crate::ability_tree::zone::OwnedZone {
                            zone: zone.clone(),
                            owner: crate::ability_tree::player::PlayerSpecifier::You {
                                #[cfg(feature = "spanned_tree")]
                                span: *owner_span,
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: owner_span.merge(&zone.node_span()),
                        }),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
            /* Otherwise, any player specifier is valid: "enchanted creature's controller graveyard" is valid */
            super::ParserRule {
                expanded: super::RuleLhs::new(&[
                    ParserNode::Player { player: dummy() }.id(),
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ApostropheS {
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }))
                    .id(),
                    ParserNode::LexerToken(Token::OwnableZone(zone)).id(),
                ]),
                merged: ParserNode::ZoneReference { zone: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::Player { player },
                        ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::ApostropheS { .. })),
                        ParserNode::LexerToken(Token::OwnableZone(zone)),
                    ] => Ok(ParserNode::ZoneReference {
                        zone: crate::ability_tree::zone::ZoneReference::OwnedZone(crate::ability_tree::zone::OwnedZone {
                            zone: zone.clone(),
                            owner: player.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: player.node_span().merge(&zone.node_span()),
                        }),
                    }),
                    _ => Err("Provided tokens do not match rule definition"),
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            },
        ]
    })
    .flatten()
    .collect::<Vec<_>>();

    [default_rules, owned_zone_rules].into_iter().flatten()
}
