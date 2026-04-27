use crate::lexer::tokens::Token;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = ParserRule> {
    [/* Treasure token */ ParserRule {
        expanded: crate::parser::rules::RuleLhs::new(&[
            ParserNode::LexerToken(Token::ArtifactSubtype(crate::ability_tree::terminals::ArtifactSubtype {
                artifact_subtype: mtg_data::ArtifactType::Treasure,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::Supertype(crate::ability_tree::terminals::Supertype {
                supertype: mtg_data::Supertype::Token,
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::TokenDefinition { token: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::ArtifactSubtype(crate::ability_tree::terminals::ArtifactSubtype {
                    artifact_subtype: mtg_data::ArtifactType::Treasure,
                    #[cfg(feature = "spanned_tree")]
                        span: treasure_span,
                })),
                ParserNode::LexerToken(Token::Supertype(crate::ability_tree::terminals::Supertype {
                    supertype: mtg_data::Supertype::Token,
                    #[cfg(feature = "spanned_tree")]
                        span: token_span,
                })),
            ] => Ok(ParserNode::TokenDefinition {
                token: crate::ability_tree::card_layout::TokenLayout {
                    name: "Treasure".to_string(),
                    card_type: crate::ability_tree::type_line::TypeLine::artifact_token(
                        &[mtg_data::ArtifactType::Treasure],
                        #[cfg(feature = "spanned_tree")]
                        treasure_span.merge(token_span),
                    ),
                    colors: crate::ability_tree::colors::Colors::empty(),
                    abilities: crate::ability_tree::ability::common::treasure_token_ability(),
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
