use super::ParserNode;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::parser::node::DummyInit;
use idris::Idris;

fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let duration_to_continuous_effect = [
        crate::ability_tree::terminals::ContinuousEffectDuration::UntilEndOfTurn,
        crate::ability_tree::terminals::ContinuousEffectDuration::UntilEndOfNextTurn,
    ]
    .into_iter()
    .map(|duration| super::ParserRule {
        from: super::RuleLhs::new(&[
            ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
            ParserNode::LexerToken(TokenKind::ContinuousEffectDuration(duration)).id(),
            ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
        ]),
        result: ParserNode::ContinuousEffect { effect: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ContinuousEffectKind { kind },
                ParserNode::LexerToken(TokenKind::ContinuousEffectDuration(duration)),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
            ] => Some(ParserNode::ContinuousEffect {
                effect: crate::ability_tree::ability::statik::continuous_effect::ContinuousEffect {
                    duration: *duration,
                    effect: kind.clone(),
                },
            }),
            _ => None,
        },
        creation_loc: super::ParserRuleDeclarationLocation::here(),
    })
    .collect::<Vec<_>>();

    let non_repetitive_rules = vec![
        /* We can have a continuous effect kind and a terminal dot,
         * it means the continuous effect last as long as the card generating it. */
        super::ParserRule {
            from: super::RuleLhs::new(&[
                ParserNode::ContinuousEffectKind { kind: dummy() }.id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)).id(),
            ]),
            result: ParserNode::ContinuousEffect { effect: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::ContinuousEffectKind { kind },
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::Dot)),
                ] => {
                    use crate::ability_tree::ability::statik::continuous_effect::ContinuousEffect;
                    Some(ParserNode::ContinuousEffect {
                        effect: ContinuousEffect {
                            duration: crate::ability_tree::terminals::ContinuousEffectDuration::ObjectStaticAbility,
                            effect: kind.clone(),
                        },
                    })
                }
                _ => None,
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    [duration_to_continuous_effect, non_repetitive_rules].into_iter().flatten()
}
