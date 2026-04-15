use super::ParserNode;
use crate::ability_tree::terminals;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let ability_kind_rules = vec![
        /* Ability as an written ability */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::Ability { ability: dummy() }.id()]),
            merged: ParserNode::AbilityKind { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Ability { ability }] => Ok(ParserNode::AbilityKind {
                    ability: crate::ability_tree::ability::AbilityKind::Written(ability.clone()),
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
            merged: ParserNode::AbilityKind { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::KeywordAbility { keyword_ability }] => Ok(ParserNode::AbilityKind {
                    ability: crate::ability_tree::ability::AbilityKind::KeywordAbility(keyword_ability.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ];

    /* Ability Word with the ability, with a em dash */
    let ability_word_to_ability = mtg_data::AbilityWord::all()
        .map(|ab_word| super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::LexerToken(Token::AbilityWord(terminals::AbilityWord {
                    ability_word: ab_word,
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::LongDash {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Ability { ability: dummy() }.id(),
            ]),
            merged: ParserNode::AbilityKind { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::AbilityWord(ab_word)),
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::LongDash { .. })),
                    ParserNode::Ability { ability },
                ] => Ok(ParserNode::AbilityKind {
                    ability: crate::ability_tree::ability::AbilityKind::AbilityWord(
                        crate::ability_tree::ability::AbilityWordAbility {
                            word: ab_word.clone(),
                            ability: ability.clone(),
                            #[cfg(feature = "spanned_tree")]
                            span: ab_word.span.merge(&ability.node_span()),
                        },
                    ),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    [ability_kind_rules, ability_word_to_ability].into_iter().flatten()
}
