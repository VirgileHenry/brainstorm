use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::object;
use boseiju_tree::ability_tree::quantifier;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "<count> <specified spell>" is a spell */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::Quantifier {
                    count: Default::default(),
                }
                .id(),
                ParserNode::SpecifiedSpell {
                    spell: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Spell {
                spell: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::Quantifier { count }, ParserNode::SpecifiedSpell { spell }] => Ok(ParserNode::Spell {
                    spell: object::Spell::Reference(object::reference::SpellReference {
                        count: count.clone(),
                        spell: spell.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.span().merge(&spell.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "another <specified spell>" is a + other spell */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedSpell {
                    spell: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Spell {
                spell: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                        #[cfg(feature = "spanned_tree")]
                            span: another_span,
                    })),
                    ParserNode::SpecifiedSpell { spell },
                ] => Ok(ParserNode::Spell {
                    spell: object::Spell::Reference(object::reference::SpellReference {
                        count: quantifier::ActiveQuantifier::Count(quantifier::CountQuantifier {
                            number: boseiju_tree::ability_tree::number::Number::Flat(
                                boseiju_tree::ability_tree::number::FixedNumber {
                                    number: 1,
                                    #[cfg(feature = "spanned_tree")]
                                    span: *another_span,
                                },
                            ),
                            #[cfg(feature = "spanned_tree")]
                            span: *another_span,
                        }),
                        spell: spell.add_factor_specifier(object::specified_object::SpellSpecifier::Another(
                            object::specified_object::AnotherObjectSpecifier {
                                #[cfg(feature = "spanned_tree")]
                                span: *another_span,
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: spell.span().merge(another_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified spell>" is a spell with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedSpell {
                spell: Default::default(),
            }
            .id()]),
            merged: ParserNode::Spell {
                spell: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedSpell { spell }] => Ok(ParserNode::Spell {
                    spell: object::Spell::Reference(object::reference::SpellReference {
                        count: quantifier::ActiveQuantifier::All(quantifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: spell.span().empty_at_start(),
                        }),
                        spell: spell.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: spell.span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <specified spell>" can be used as a spell reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishDemonstrative(intermediate::EnglishDemonstrative::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedSpell {
                    spell: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::Spell {
                spell: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishDemonstrative(intermediate::EnglishDemonstrative::This {
                        #[cfg(feature = "spanned_tree")]
                            span: start_span,
                    })),
                    ParserNode::SpecifiedSpell {
                        #[cfg(feature = "spanned_tree")]
                        spell,
                        ..
                    },
                ] => Ok(ParserNode::Spell {
                    spell: object::Spell::SelfReferencing(object::SelfReferencing {
                        #[cfg(feature = "spanned_tree")]
                        span: spell.span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
