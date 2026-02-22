use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::{TokenKind, non_terminals};
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let default_rules = vec![super::ParserRule {
        from: super::RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::GlobalZone(non_terminals::GlobalZone::TheBattlefield)).id(),
        ]),
        result: ParserNode::ZoneReference { zone: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(TokenKind::GlobalZone(non_terminals::GlobalZone::TheBattlefield))] => {
                Some(ParserNode::ZoneReference {
                    zone: crate::ability_tree::zone::ZoneReference::TheBattlefield,
                })
            }
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    }];

    let owned_zone_rules = [
        crate::ability_tree::zone::OwnableZone::Graveyard,
        crate::ability_tree::zone::OwnableZone::Hand,
        crate::ability_tree::zone::OwnableZone::Library,
    ]
    .into_iter()
    .map(|zone| {
        [terminals::OwnerSpecifier::YouOwn, terminals::OwnerSpecifier::ObjectOwner]
            .into_iter()
            .map(move |owner| super::ParserRule {
                from: super::RuleLhs::new(&[
                    ParserNode::LexerToken(TokenKind::OwnerSpecifier(owner)).id(),
                    ParserNode::LexerToken(TokenKind::OwnableZone(zone)).id(),
                ]),
                result: ParserNode::ZoneReference { zone: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(TokenKind::OwnerSpecifier(owner)),
                        ParserNode::LexerToken(TokenKind::OwnableZone(zone)),
                    ] => Some(ParserNode::ZoneReference {
                        zone: crate::ability_tree::zone::ZoneReference::OwnedZone(crate::ability_tree::zone::OwnedZone {
                            zone: zone.clone(),
                            owner: owner.clone(),
                        }),
                    }),
                    _ => None,
                },
                creation_loc: super::ParserRuleDeclarationLocation::here(),
            })
    })
    .flatten()
    .collect::<Vec<_>>();
    [default_rules, owned_zone_rules].into_iter().flatten()
}
