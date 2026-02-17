use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::TokenKind;
use crate::parser::node::DummyInit;
use idris::Idris;

fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let default_rules = vec![super::ParserRule {
        from: super::RuleLhs::new(&[ParserNode::LexerToken(TokenKind::Zone(crate::ability_tree::zone::Zone::Battlefield)).id()]),
        result: ParserNode::Zone { zone: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[ParserNode::LexerToken(TokenKind::Zone(crate::ability_tree::zone::Zone::Battlefield))] => Some(ParserNode::Zone {
                zone: crate::ability_tree::zone::ZoneReference::TheBattlefield,
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    }];
    let owned_zone_rules = [
        crate::ability_tree::zone::Zone::Graveyard,
        crate::ability_tree::zone::Zone::Hand,
        crate::ability_tree::zone::Zone::Library,
    ]
    .into_iter()
    .map(|zone| {
        [terminals::OwnerSpecifier::YouOwn, terminals::OwnerSpecifier::ObjectOwner]
            .into_iter()
            .map(move |owner| super::ParserRule {
                from: super::RuleLhs::new(&[
                    ParserNode::LexerToken(TokenKind::OwnerSpecifier(owner)).id(),
                    ParserNode::LexerToken(TokenKind::Zone(zone)).id(),
                ]),
                result: ParserNode::Zone { zone: dummy() }.id(),
                reduction: |nodes: &[ParserNode]| match &nodes {
                    &[
                        ParserNode::LexerToken(TokenKind::OwnerSpecifier(owner)),
                        ParserNode::LexerToken(TokenKind::Zone(zone)),
                    ] => Some(ParserNode::Zone {
                        zone: crate::ability_tree::zone::ZoneReference::OwnedZone {
                            zone: zone.clone(),
                            owner: owner.clone(),
                        },
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
