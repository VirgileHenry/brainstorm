use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* Ability as an written ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::WrittenAbility { ability: dummy() }.id()]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::WrittenAbility { ability }] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::Written(ability.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Ability as a keyword ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::KeywordAbility {
                keyword_ability: dummy(),
            }
            .id()]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::KeywordAbility { keyword_ability }] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::KeywordAbility(keyword_ability.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Ability as an Ability word with the em dash keyword ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::AbilityWord { ability_word: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::LongDash {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::WrittenAbility { ability: dummy() }.id(),
            ]),
            merged: ParserNode::Ability { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::AbilityWord { ability_word },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::LongDash { .. })),
                    ParserNode::WrittenAbility { ability },
                ] => Ok(ParserNode::Ability {
                    ability: crate::ability_tree::ability::Ability::AbilityWord(
                        crate::ability_tree::ability::AbilityWordAbility {
                            word: ability_word.clone(),
                            ability: ability.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: ability_word.node_span().merge(&ability.node_span()),
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
