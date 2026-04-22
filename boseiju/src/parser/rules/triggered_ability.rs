use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<trigger_cond>, <spell ability>" make the structure for triggered abilities. */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::TriggerCondition { condition: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::WrittenAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::TriggerCondition { condition },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Comma { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::WrittenAbility {
                    ability: crate::ability_tree::ability::WrittenAbility::Triggered(
                        crate::ability_tree::ability::triggered::TriggeredAbility {
                            condition: condition.clone(),
                            effect: ability.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: condition.span.merge(&ability.span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
