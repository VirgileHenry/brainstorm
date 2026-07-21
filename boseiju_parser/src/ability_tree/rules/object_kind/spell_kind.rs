use crate::ParserNode;
use crate::ability_tree::rules::ParserRule;
use crate::ability_tree::rules::ParserRuleDeclarationLocation;
use crate::ability_tree::rules::RuleLhs;
use boseiju_lexer::Token;
use boseiju_lexer::intermediate;
use boseiju_tree::ability_tree::object;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use boseiju_span::Spanned;

pub fn rules() -> impl Iterator<Item = crate::ability_tree::rules::ParserRule> {
    [
        /* "spell" is the default spell kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Spell {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id()]),
            merged: ParserNode::SpellKind {
                spell: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Spell {
                        #[cfg(feature = "spanned_tree")]
                        span,
                    })),
                ] => Ok(ParserNode::SpellKind {
                    spell: object::kind::SpellKind::Spell {
                        #[cfg(feature = "spanned_tree")]
                        span: *span,
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<specified permanent>" can be used as a spell kind */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpecifiedPermanent {
                permanent: Default::default(),
            }
            .id()]),
            merged: ParserNode::SpellKind {
                spell: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpecifiedPermanent { permanent }] => Ok(ParserNode::SpellKind {
                    spell: object::kind::SpellKind::Permanent(permanent.clone()),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<spell kind> or <spell kind>" makes a one among kind */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::SpellKind {
                    spell: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellKind {
                    spell: Default::default(),
                }
                .id(),
            ]),
            merged: ParserNode::SpellKind {
                spell: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::SpellKind { spell: c1 },
                    ParserNode::LexerToken(Token::EnglishConjunction(intermediate::EnglishConjunction::Or { .. })),
                    ParserNode::SpellKind { spell: c2 },
                ] => Ok(ParserNode::SpellKind {
                    spell: object::kind::SpellKind::OneAmong(object::OneAmong {
                        references: {
                            let mut references = boseiju_tree::HeapArrayVec::new();
                            references.push(c1.clone());
                            references.push(c2.clone());
                            references
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: c1.span().merge(&c2.span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
