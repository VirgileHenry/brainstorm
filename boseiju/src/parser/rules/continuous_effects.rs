use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let duration_to_continuous_effect = [
        crate::ability_tree::time::ForwardDuration::UntilEndOfTurn {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
        crate::ability_tree::time::ForwardDuration::UntilEndOfYourNextTurn {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        },
    ]
    .into_iter()
    .map(|duration| super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
            ParserNode::LexerToken(Token::ForwardDuration(duration)).id(),
        ]),
        merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ContinuousEffectKind { kind },
                ParserNode::LexerToken(Token::ForwardDuration(duration)),
            ] => Ok(ParserNode::ContinuousEffect {
                effect: crate::ability_tree::ability::statik::continuous_effect::ContinuousEffect {
                    duration: *duration,
                    effect: kind.clone(),
                    #[cfg(feature = "spanned_tree")]
                    span: kind.span().merge(&duration.span()),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let non_repetitive_rules = vec![
        /* We can have a continuous effect kind alone,
         * it means the continuous effect last as long as the card generating it. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::ContinuousEffectKind { kind: dummy() }.id()]),
            merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::ContinuousEffectKind { kind }] => {
                    use crate::ability_tree::ability::statik::continuous_effect::ContinuousEffect;
                    Ok(ParserNode::ContinuousEffect {
                        effect: ContinuousEffect {
                            duration: crate::ability_tree::time::ForwardDuration::ObjectLifetime {
                                #[cfg(feature = "spanned_tree")]
                                span: kind.span().empty_at_end(),
                            },
                            effect: kind.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: kind.span(),
                        },
                    })
                }
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [duration_to_continuous_effect, non_repetitive_rules].into_iter().flatten()
}
