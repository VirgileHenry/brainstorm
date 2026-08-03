use crate::ability_tree::rules::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "at <instant>, <spell ability>" can be a delayed triggered ability, thus a generate dta imperative. */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::At {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::IncomingInstant {
                    instant: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellAbility {
                    ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishPreprosition(intermediate::EnglishPreprosition::At {
                        #[cfg(feature = "spanned_tree")]
                            span: at_span,
                    })),
                    ParserNode::IncomingInstant { instant },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::Comma { .. })),
                    ParserNode::SpellAbility { ability },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::GenerateDelayedTriggeredAbility(
                        boseiju_tree::ability_tree::imperative::GenerateDelayedTriggeredAbilityImperative {
                            ability: boseiju_tree::ability_tree::ability::triggered::DelayedTriggerAbility {
                                instant: instant.clone(),
                                effect: ability.clone(),
                                #[cfg(feature = "spanned_tree")]
                                span: ability.span().merge(at_span),
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: ability.span().merge(at_span),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
