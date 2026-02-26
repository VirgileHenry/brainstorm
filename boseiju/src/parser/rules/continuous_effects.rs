use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let duration_to_continuous_effect = [
        crate::ability_tree::time::ForwardDuration::UntilEndOfTurn,
        crate::ability_tree::time::ForwardDuration::UntilEndOfYourNextTurn,
    ]
    .into_iter()
    .map(|duration| super::ParserRule {
        expanded: super::RuleLhs::new(&[
            ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::ForwardDuration(duration)).id(),
        ]),
        merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ContinuousEffectKind { kind },
                ParserNode::LexerToken(TokenKind::ForwardDuration(duration)),
            ] => Ok(ParserNode::ContinuousEffect {
                effect: crate::ability_tree::ability::statik::continuous_effect::ContinuousEffect {
                    duration: *duration,
                    effect: kind.clone(),
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
                            duration: crate::ability_tree::time::ForwardDuration::ObjectLifetime,
                            effect: kind.clone(),
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
