use super::ParserNode;
use crate::lexer::tokens::{TokenKind, non_terminals};
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)).id(),
            ParserNode::Event { event: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)).id(),
            ParserNode::EventReplacement { replacement: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Instead)).id(),
        ]),
        merged: ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::If)),
                ParserNode::Event { event },
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Comma)),
                ParserNode::EventReplacement { replacement },
                ParserNode::LexerToken(TokenKind::EnglishKeyword(non_terminals::EnglishKeyword::Instead)),
            ] => {
                use crate::ability_tree::ability::statik::continuous_effect::continuous_effect_kind;
                Ok(ParserNode::ContinuousEffectKind {
                    kind: continuous_effect_kind::ContinuousEffectKind::ReplacementEffect(
                        continuous_effect_kind::ContinuousEffectReplacementEvent {
                            replaced_event: event.clone(),
                            replaced_by: replacement.clone(),
                        },
                    ),
                })
            }
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
