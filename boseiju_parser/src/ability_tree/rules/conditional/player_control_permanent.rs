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
    [/* "<player> controls <permanent reference>" condition */ ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::PlayerPassive {
                player: Default::default(),
            }
            .id(),
            ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                token: intermediate::EnglishVerb::Control {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
                tense: boseiju_lexer::Tense::BaseForm,
            }))
            .id(),
            ParserNode::PermanentActive {
                permanent: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::Condition {
            condition: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::PlayerPassive { player },
                ParserNode::LexerToken(Token::EnglishVerb(intermediate::TensedEnglishVerb {
                    token: intermediate::EnglishVerb::Control { .. },
                    tense: boseiju_lexer::Tense::BaseForm,
                })),
                ParserNode::PermanentActive { permanent },
            ] => Ok(ParserNode::Condition {
                condition: conditional::Condition::PlayerControlsObject(conditional::ConditionPlayerControlsPermanent {
                    player: player.clone(),
                    permanent: permanent.clone(),
                    #[cfg(feature = "spanned_tree")]
                    span: player.span().merge(&permanent.span()),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
