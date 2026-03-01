use super::ParserNode;
use super::ParserRule;
use super::ParserRuleDeclarationLocation;
use super::RuleLhs;
use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::non_terminals;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    let standalone_keyword_abilities = crate::ability_tree::ability::keyword::all_standalone_kw_abilities()
        .map(|keyword| ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(TokenKind::KeywordAbility(keyword)).id()]),
            merged: ParserNode::KeywordAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::LexerToken(TokenKind::KeywordAbility(keyword))] => Ok(ParserNode::KeywordAbility {
                    ability: crate::ability_tree::ability::keyword::keyword_to_abilities(*keyword)?,
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        })
        .collect::<Vec<_>>();

    let keyword_abilities_with_extra_nodes = vec![
        /* Enchant an object specifiers (creature, artifact, creature you control...) */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::KeywordAbility(mtg_data::KeywordAbility::Enchant)).id(),
                ParserNode::ObjectSpecifiers { specifiers: dummy() }.id(),
            ]),
            merged: ParserNode::KeywordAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::KeywordAbility(mtg_data::KeywordAbility::Enchant)),
                    ParserNode::ObjectSpecifiers { specifiers },
                ] => Ok(ParserNode::KeywordAbility {
                    ability: crate::ability_tree::ability::KeywordAbility {
                        keyword: crate::ability_tree::ability::keyword::ExpandedKeywordAbility::Enchant(
                            crate::ability_tree::ability::keyword::EnchantKeywordAbility {
                                enchantable_object: specifiers.clone(),
                            },
                        ),
                        /* Fixme */
                        ability: crate::ability_tree::ability::Ability::Spell(
                            crate::ability_tree::ability::spell::SpellAbility {
                                effects: crate::utils::HeapArrayVec::new(),
                            },
                        ),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Ward with a mana cost */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::KeywordAbility(mtg_data::KeywordAbility::Ward)).id(),
                ParserNode::ManaCost { mana_cost: dummy() }.id(),
            ]),
            merged: ParserNode::KeywordAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::KeywordAbility(mtg_data::KeywordAbility::Enchant)),
                    ParserNode::ManaCost { mana_cost },
                ] => Ok(ParserNode::KeywordAbility {
                    ability: crate::ability_tree::ability::KeywordAbility {
                        keyword: crate::ability_tree::ability::keyword::ExpandedKeywordAbility::Ward(
                            crate::ability_tree::ability::keyword::WardKeywordAbility {
                                cost: crate::ability_tree::cost::Cost::ManaCost(mana_cost.clone()),
                            },
                        ),
                        /* Fixme */
                        ability: crate::ability_tree::ability::Ability::Spell(
                            crate::ability_tree::ability::spell::SpellAbility {
                                effects: crate::utils::HeapArrayVec::new(),
                            },
                        ),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* Ward with a different cost require a long hyphen */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(TokenKind::KeywordAbility(mtg_data::KeywordAbility::Ward)).id(),
                ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::LongDash)).id(),
                ParserNode::Cost { cost: dummy() }.id(),
            ]),
            merged: ParserNode::KeywordAbility { ability: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(TokenKind::KeywordAbility(mtg_data::KeywordAbility::Enchant)),
                    ParserNode::LexerToken(TokenKind::ControlFlow(non_terminals::ControlFlow::LongDash)),
                    ParserNode::Cost { cost },
                ] => Ok(ParserNode::KeywordAbility {
                    ability: crate::ability_tree::ability::KeywordAbility {
                        keyword: crate::ability_tree::ability::keyword::ExpandedKeywordAbility::Ward(
                            crate::ability_tree::ability::keyword::WardKeywordAbility { cost: cost.clone() },
                        ),
                        /* Fixme */
                        ability: crate::ability_tree::ability::Ability::Spell(
                            crate::ability_tree::ability::spell::SpellAbility {
                                effects: crate::utils::HeapArrayVec::new(),
                            },
                        ),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ];

    [standalone_keyword_abilities, keyword_abilities_with_extra_nodes]
        .into_iter()
        .flatten()
}
