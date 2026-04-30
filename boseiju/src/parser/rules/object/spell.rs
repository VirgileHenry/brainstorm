use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [
        /* "<count> <specified spell>" is a spell */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::CountSpecifier { count: dummy() }.id(),
                ParserNode::SpecifiedSpell { spell: dummy() }.id(),
            ]),
            merged: ParserNode::Spell { spell: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::CountSpecifier { count }, ParserNode::SpecifiedSpell { spell }] => Ok(ParserNode::Spell {
                    spell: object::Spell::Reference(object::reference::SpellReference {
                        count: count.clone(),
                        spell: spell.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: count.node_span().merge(&spell.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "another <specified spell>" is a + other spell */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedSpell { spell: dummy() }.id(),
            ]),
            merged: ParserNode::Spell { spell: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                        #[cfg(feature = "spanned_tree")]
                            span: another_span,
                    })),
                    ParserNode::SpecifiedSpell { spell },
                ] => Ok(ParserNode::Spell {
                    spell: object::Spell::Reference(object::reference::SpellReference {
                        count: object::CountSpecifier::A {
                            #[cfg(feature = "spanned_tree")]
                            span: *another_span,
                        },
                        spell: spell.add_factor_specifier(object::specified_object::SpellSpecifier::Another(
                            object::specified_object::AnotherObjectSpecifier {
                                #[cfg(feature = "spanned_tree")]
                                span: *another_span,
                            },
                        )),
                        #[cfg(feature = "spanned_tree")]
                        span: spell.node_span().merge(another_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified spell>" is a spell with an implicit "all" */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedSpell { spell: dummy() }.id()]),
            merged: ParserNode::Spell { spell: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedSpell { spell }] => Ok(ParserNode::Spell {
                    spell: object::Spell::Reference(object::reference::SpellReference {
                        count: object::CountSpecifier::All {
                            #[cfg(feature = "spanned_tree")]
                            span: spell.node_span().empty_at_start(),
                        },
                        spell: spell.clone(),
                        #[cfg(feature = "spanned_tree")]
                        span: spell.node_span(),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "this <specified spell>" can be used as a spell reference */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpecifiedSpell { spell: dummy() }.id(),
            ]),
            merged: ParserNode::Spell { spell: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::This {
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
                        span: spell.node_span().merge(start_span),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
