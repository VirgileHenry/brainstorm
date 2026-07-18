use crate::ability_tree::ParserNode;
use crate::ability_tree::node::MultipleKeywordAbilities;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate::ControlFlow;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    /* Multiple keyword abilities can be found separated by commas. */
    [
        /* "<keyword ability>" (1 rep) */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::KeywordAbility {
                keyword_ability: Default::default(),
            }
            .id()]),
            merged: ParserNode::MultipleKeywordAbilities {
                abilities: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::KeywordAbility { keyword_ability }] => Ok(ParserNode::MultipleKeywordAbilities {
                    abilities: MultipleKeywordAbilities {
                        abilities: {
                            let mut abilities = boseiju_tree::HeapArrayVec::new();
                            abilities.push(keyword_ability.clone());
                            abilities
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: keyword_ability.span(),
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
                    keyword_ability: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::MultipleKeywordAbilities {
                abilities: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::KeywordAbility { keyword_ability: ab1 },
                    ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma { .. })),
                    ParserNode::KeywordAbility { keyword_ability: ab2 },
                ] => Ok(ParserNode::MultipleKeywordAbilities {
                    abilities: MultipleKeywordAbilities {
                        abilities: {
                            let mut abilities = boseiju_tree::HeapArrayVec::new();
                            for ab in [ab1.clone(), ab2.clone()].into_iter() {
                                abilities.push(ab.clone());
                            }
                            abilities
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: ab1.span().merge(&ab2.span()),
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
                    keyword_ability: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::MultipleKeywordAbilities {
                abilities: Default::default(),
            }
            .id(),
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
                            let mut abilities = boseiju_tree::HeapArrayVec::new();
                            for ab in [ab1.clone(), ab2.clone(), ab3.clone()].into_iter() {
                                abilities.push(ab.clone());
                            }
                            abilities
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: ab1.span().merge(&ab3.span()),
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
                    keyword_ability: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::ControlFlow(ControlFlow::Comma {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::KeywordAbility {
                    keyword_ability: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::MultipleKeywordAbilities {
                abilities: Default::default(),
            }
            .id(),
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
                            let mut abilities = boseiju_tree::HeapArrayVec::new();
                            for ab in [ab1.clone(), ab2.clone(), ab3.clone(), ab4.clone()].into_iter() {
                                abilities.push(ab.clone());
                            }
                            abilities
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: ab1.span().merge(&ab4.span()),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
