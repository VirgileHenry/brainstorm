use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                span: Default::default(),
            }))
            .id(),
            ParserNode::Event { event: dummy() }.id(),
            ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                span: Default::default(),
            }))
            .id(),
            ParserNode::EventReplacement { replacement: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Instead {
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If { span: if_span })),
                ParserNode::Event { event },
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                ParserNode::EventReplacement { replacement },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Instead { span: instead_span })),
            ] => {
                use crate::ability_tree::ability::statik::continuous_effect;
                Ok(ParserNode::ContinuousEffectKind {
                    kind: continuous_effect::ContinuousEffectKind::ReplacementEffect(
                        continuous_effect::ContinuousEffectReplacementEvent {
                            replaced_event: event.clone(),
                            replaced_by: replacement.clone(),
                            span: if_span.merge(instead_span),
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
