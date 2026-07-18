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
        /* "<spell ability> for each <game state number>" is a "for each" imperative */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::SpellAbility {
                    ability: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::ForEach {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::GameStateNumber {
                    number: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::ImperativeKind {
                imperative: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::SpellAbility { ability },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediate::EnglishKeyword::ForEach { .. })),
                    ParserNode::GameStateNumber { number },
                ] => Ok(ParserNode::ImperativeKind {
                    imperative: boseiju_tree::ability_tree::imperative::ImperativeKind::ForEach(
                        boseiju_tree::ability_tree::imperative::ForEachImperative {
                            ability: ability.clone(),
                            for_each: number.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: ability.span().merge(&number.span()),
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
