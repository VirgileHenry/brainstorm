use super::ParserNode;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* Ability as an written ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::WrittenAbility {
                ability: Default::default(),
            }
            .id()]),
            merged: ParserNode::Ability {
                ability: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::WrittenAbility { ability }] => Ok(ParserNode::Ability {
                    ability: boseiju_tree::ability_tree::ability::Ability::Written(ability.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Ability as a keyword ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::KeywordAbility {
                keyword_ability: Default::default(),
            }
            .id()]),
            merged: ParserNode::Ability {
                ability: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::KeywordAbility { keyword_ability }] => Ok(ParserNode::Ability {
                    ability: boseiju_tree::ability_tree::ability::Ability::KeywordAbility(keyword_ability.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Ability as an Ability word with the em dash keyword ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::AbilityWord {
                    ability_word: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::LongDash {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::WrittenAbility {
                    ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Ability {
                ability: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::AbilityWord { ability_word },
                    ParserNode::LexerToken(Token::ControlFlow(intermediate::ControlFlow::LongDash { .. })),
                    ParserNode::WrittenAbility { ability },
                ] => Ok(ParserNode::Ability {
                    ability: boseiju_tree::ability_tree::ability::Ability::AbilityWord(
                        boseiju_tree::ability_tree::ability::AbilityWordAbility {
                            word: ability_word.clone(),
                            ability: ability.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: ability_word.span().merge(&ability.span()),
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
