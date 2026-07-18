use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use boseiju_lexer::Token;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = ParserRule> {
    [/* Treasure token */ ParserRule {
        expanded: crate::ability_tree::rules::RuleLhs::new(&[
            ParserNode::LexerToken(Token::ArtifactSubtype(boseiju_lexer::terminal::ArtifactSubtype {
                artifact_subtype: mtg_data::ArtifactType::Treasure,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::Supertype(boseiju_lexer::terminal::Supertype {
                supertype: mtg_data::Supertype::Token,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::TokenDefinition {
            token: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::ArtifactSubtype(boseiju_lexer::terminal::ArtifactSubtype {
                    artifact_subtype: mtg_data::ArtifactType::Treasure,
                    #[cfg(feature = "spanned_tree")]
                        span: treasure_span,
                })),
                ParserNode::LexerToken(Token::Supertype(boseiju_lexer::terminal::Supertype {
                    supertype: mtg_data::Supertype::Token,
                    #[cfg(feature = "spanned_tree")]
                        span: token_span,
                })),
            ] => Ok(ParserNode::TokenDefinition {
                token: boseiju_tree::card::layout::TokenLayout {
                    name: "Treasure".to_string(),
                    card_type: boseiju_tree::ability_tree::type_line::TypeLine::artifact_token(
                        &[mtg_data::ArtifactType::Treasure],
                        #[cfg(feature = "spanned_tree")]
                        treasure_span.merge(token_span),
                    ),
                    color_identity: boseiju_tree::ability_tree::colors::Colors::empty(),
                    abilities: boseiju_tree::ability_tree::ability::common::treasure_token_ability(),
                    #[cfg(feature = "spanned_tree")]
                    span: treasure_span.merge(token_span),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
