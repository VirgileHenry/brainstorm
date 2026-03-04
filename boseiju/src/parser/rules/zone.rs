use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let default_rules = vec![super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::LexerToken(Token::GlobalZone(intermediates::GlobalZone::TheBattlefield {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::ZoneReference { zone: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(Token::GlobalZone(intermediates::GlobalZone::TheBattlefield {
                #[cfg(feature = "spanned_tree")] span }))] => {
                Ok(ParserNode::ZoneReference {
                    zone: crate::ability_tree::zone::ZoneReference::TheBattlefield {
                        #[cfg(feature = "spanned_tree")] span: *span },
                })
            }
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    }];

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
            terminals::OwnerSpecifier::YouOwn {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
            terminals::OwnerSpecifier::ObjectOwner {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ]
        .into_iter()
        .map(move |owner| super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::OwnerSpecifier(owner)).id(),
                ParserNode::LexerToken(Token::OwnableZone(zone)).id(),
            ]),
            merged: ParserNode::ZoneReference { zone: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::OwnerSpecifier(owner)),
                    ParserNode::LexerToken(Token::OwnableZone(zone)),
                ] => Ok(ParserNode::ZoneReference {
                    zone: crate::ability_tree::zone::ZoneReference::OwnedZone(crate::ability_tree::zone::OwnedZone {
                        zone: zone.clone(),
                        owner: owner.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: owner.span().merge(&zone.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
    })
    .flatten()
    .collect::<Vec<_>>();
    [default_rules, owned_zone_rules].into_iter().flatten()
}
