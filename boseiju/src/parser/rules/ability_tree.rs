use super::ParserNode;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<ability>." is an ability tree on its own */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::Ability { ability: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::AbilityTree { tree: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::Ability { ability },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::AbilityTree {
                    tree: {
                        let mut abilities = crate::utils::HeapArrayVec::new();
                        abilities.push(ability.clone());
                        crate::AbilityTree {
                            abilities,
                            #[cfg(feature = "spanned_tree")]
                            span: ability.node_span().merge(end_span),
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* "<multiple keyword abilities>" is an ab tree */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[ParserNode::MultipleKeywordAbilities {
                abilities: dummy(),
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }
            .id()]),
            merged: ParserNode::AbilityTree { tree: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::MultipleKeywordAbilities {
                        abilities,
                        #[cfg(feature = "spanned_tree")]
                        span,
                    },
                ] => Ok(ParserNode::AbilityTree {
                    tree: {
                        let mut ab_tree = crate::utils::HeapArrayVec::new();
                        for ab in abilities.iter() {
                            ab_tree.push(crate::ability_tree::ability::Ability::KeywordAbility(ab.clone()));
                        }
                        crate::AbilityTree {
                            abilities: ab_tree,
                            #[cfg(feature = "spanned_tree")]
                            span: *span,
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Abilities separated by new lines can be merged into a single ability tree */
        /* Fixme: fixed number of abilities per cards ? */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::AbilityTree { tree: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::NewLine {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::Ability { ability: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::AbilityTree { tree: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::AbilityTree { tree },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::NewLine { .. })),
                    ParserNode::Ability { ability },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::Dot {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::AbilityTree {
                    tree: {
                        let mut abilities = tree.abilities.clone();
                        abilities.push(ability.clone());
                        crate::AbilityTree {
                            abilities,
                            #[cfg(feature = "spanned_tree")]
                            span: tree.span.merge(end_span),
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
        /* Abilities separated by new lines can be merged into a single ability tree */
        /* Fixme: fixed number of abilities per cards ? */
        super::ParserRule {
            expanded: super::RuleLhs::new(&[
                ParserNode::AbilityTree { tree: dummy() }.id(),
                ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::NewLine {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::MultipleKeywordAbilities {
                    abilities: dummy(),
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::AbilityTree { tree: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::AbilityTree { tree },
                    ParserNode::LexerToken(Token::ControlFlow(intermediates::ControlFlow::NewLine { .. })),
                    ParserNode::MultipleKeywordAbilities {
                        abilities,
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    },
                ] => Ok(ParserNode::AbilityTree {
                    tree: {
                        let mut ab_tree = tree.abilities.clone();
                        for ab in abilities.iter() {
                            ab_tree.push(crate::ability_tree::ability::Ability::KeywordAbility(ab.clone()));
                        }
                        crate::AbilityTree {
                            abilities: ab_tree,
                            #[cfg(feature = "spanned_tree")]
                            span: tree.node_span().merge(end_span),
                        }
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: super::ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
