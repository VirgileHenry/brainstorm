mod etb_replacement;

use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let default_replacement_effects = vec![super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Event { event: dummy() }.id(),
            ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::EventReplacement { replacement: dummy() }.id(),
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Instead {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::If {
                    #[cfg(feature = "spanned_tree")]
                        span: if_span,
                })),
                ParserNode::Event { event },
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                ParserNode::EventReplacement { replacement },
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Instead {
                    #[cfg(feature = "spanned_tree")]
                        span: instead_span,
                })),
            ] => {
                use crate::ability_tree::ability::statik::continuous_effect;
                Ok(ParserNode::ContinuousEffect {
                    effect: continuous_effect::ContinuousEffect {
                        effect: continuous_effect::ContinuousEffectKind::ReplacementEffect(
                            continuous_effect::ContinuousEffectReplacementEvent {
                                replaced_event: event.clone(),
                                replaced_by: replacement.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: if_span.merge(instead_span),
                            },
                        ),
                        #[cfg(feature = "spanned_tree")]
                        span: if_span.merge(instead_span),
                    },
                })
            }
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    }];

    [default_replacement_effects, etb_replacement::rules().collect::<Vec<_>>()]
        .into_iter()
        .flatten()
}
