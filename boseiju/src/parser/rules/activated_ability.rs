use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<cost>: <spell ability>" makes an activated ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Cost { cost: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Colons {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::WrittenAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Cost { cost },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Colons { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::WrittenAbility {
                    ability: crate::ability_tree::ability::WrittenAbility::Activated(
                        crate::ability_tree::ability::activated::ActivatedAbility {
                            effect: ability.clone(),
                            cost: cost.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: cost.node_span().merge(&ability.span),
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
