use super::ParserNode;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<cost>: <spell ability>" makes an activated ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Cost {
                    cost: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Colons {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility {
                    ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::WrittenAbility {
                ability: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Cost { cost },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Colons { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::WrittenAbility {
                    ability: boseiju_tree::ability_tree::ability::WrittenAbility::Activated(
                        boseiju_tree::ability_tree::ability::activated::ActivatedAbility {
                            effect: ability.clone(),
                            cost: cost.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: cost.span().merge(&ability.span),
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
