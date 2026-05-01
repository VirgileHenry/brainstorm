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
        /* "spell" is the default spell kind */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Spell {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::SpellKind { spell: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Spell {
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
            expanded: RuleLhs::new(&[ParserNode::SpecifiedPermanent { permanent: dummy() }.id()]),
            merged: ParserNode::SpellKind { spell: dummy() }.id(),
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
                ParserNode::SpellKind { spell: dummy() }.id(),
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
                ParserNode::SpellKind { spell: dummy() }.id(),
            ]),
            merged: ParserNode::SpellKind { spell: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::SpellKind { spell: c1 },
                    ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Or { .. })),
                    ParserNode::SpellKind { spell: c2 },
                ] => Ok(ParserNode::SpellKind {
                    spell: object::kind::SpellKind::OneAmong(object::OneAmong {
                        references: {
                            let mut references = crate::utils::HeapArrayVec::new();
                            references.push(c1.clone());
                            references.push(c2.clone());
                            references
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: c1.node_span().merge(&c2.node_span()),
                    }),
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
