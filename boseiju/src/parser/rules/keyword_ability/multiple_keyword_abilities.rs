use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates::ControlFlow;
use crate::parser::ParserNode;
use crate::parser::node::MultipleKeywordAbilities;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* Multiple keyword abilities can be found separated by commas. */
    [
        /* "<keyword ability>" (1 rep) */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::KeywordAbility {
                keyword_ability: dummy(),
            }
            .id()]),
            merged: ParserNode::MultipleKeywordAbilities { abilities: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::KeywordAbility { keyword_ability }] => Ok(ParserNode::MultipleKeywordAbilities {
                    abilities: MultipleKeywordAbilities {
                        abilities: {
                            let mut abilities = crate::utils::HeapArrayVec::new();
                            abilities.push(keyword_ability.clone());
                            abilities
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: keyword_ability.node_span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<keyword ability>, <keyword ability>" (2 rep) */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
            ]),
            merged: ParserNode::MultipleKeywordAbilities { abilities: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::KeywordAbility { keyword_ability: ab1 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::KeywordAbility { keyword_ability: ab2 },
                ] => Ok(ParserNode::MultipleKeywordAbilities {
                    abilities: MultipleKeywordAbilities {
                        abilities: {
                            let mut abilities = crate::utils::HeapArrayVec::new();
                            for ab in [ab1.clone(), ab2.clone()].into_iter() {
                                abilities.push(ab.clone());
                            }
                            abilities
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: ab1.node_span().merge(&ab2.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<keyword ability>, <keyword ability>, <keyword ability>" (3 rep) */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
            ]),
            merged: ParserNode::MultipleKeywordAbilities { abilities: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::KeywordAbility { keyword_ability: ab1 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::KeywordAbility { keyword_ability: ab2 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::KeywordAbility { keyword_ability: ab3 },
                ] => Ok(ParserNode::MultipleKeywordAbilities {
                    abilities: MultipleKeywordAbilities {
                        abilities: {
                            let mut abilities = crate::utils::HeapArrayVec::new();
                            for ab in [ab1.clone(), ab2.clone(), ab3.clone()].into_iter() {
                                abilities.push(ab.clone());
                            }
                            abilities
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: ab1.node_span().merge(&ab3.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<keyword ability>, <keyword ability>, <keyword ability>, <keyword ability>" (4 rep) */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: dummy(),
                }
                .id(),
            ]),
            merged: ParserNode::MultipleKeywordAbilities { abilities: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::KeywordAbility { keyword_ability: ab1 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::KeywordAbility { keyword_ability: ab2 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::KeywordAbility { keyword_ability: ab3 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::KeywordAbility { keyword_ability: ab4 },
                ] => Ok(ParserNode::MultipleKeywordAbilities {
                    abilities: MultipleKeywordAbilities {
                        abilities: {
                            let mut abilities = crate::utils::HeapArrayVec::new();
                            for ab in [ab1.clone(), ab2.clone(), ab3.clone(), ab4.clone()].into_iter() {
                                abilities.push(ab.clone());
                            }
                            abilities
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: ab1.node_span().merge(&ab4.node_span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
