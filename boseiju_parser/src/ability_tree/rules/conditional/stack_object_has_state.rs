use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::conditional;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::SpellPassive {
                spell: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                token: intermediate::EnglishVerb::Be {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
                tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
            }))
            .id(),
            ParserNode::LexerToken(Token::CardState(boseiju_lexer::intermediate::CardState::Countered {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
        ]),
        merged: ParserNode::Condition {
            condition: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::SpellPassive { spell },
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Be { .. },
                    tense: boseiju_lexer::Tense::ThirdPersonSingularPresent,
                })),
                ParserNode::LexerToken(Token::CardState(boseiju_lexer::intermediate::CardState::Countered {
                    #[cfg(feature = "spanned_tree")]
                        span: countered_span,
                })),
            ] => Ok(ParserNode::Condition {
                condition: conditional::Condition::StackObjectHasState(conditional::ConditionStackObjectHasState {
                    stack_obj: spell.clone(),
                    state: boseiju_tree::ability_tree::state::StackObjectState::Countered(
                        boseiju_tree::ability_tree::state::CounteredState {
                            #[cfg(feature = "spanned_tree")]
                            span: *countered_span,
                        },
                    ),
                    has_state: true,
                    #[cfg(feature = "spanned_tree")]
                    span: spell.span().merge(countered_span),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
