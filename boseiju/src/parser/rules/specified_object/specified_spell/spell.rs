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
        /* "<spell kind>" makes a specified spell  */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpellKind { spell: dummy() }.id()]),
            merged: ParserNode::SpecifiedSpell { spell: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpellKind { spell }] => Ok(ParserNode::SpecifiedSpell {
                    spell: object::specified_object::SpecifiedSpell {
                        kind: spell.clone(),
                        specifiers: None,
                        #[cfg(feature = "spanned_tree")]
                        span: spell.node_span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<spell kind> spell" makes a specified spell  */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::SpellKind { spell: dummy() }.id(),
                ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Spell {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::SpecifiedSpell { spell: dummy() }.id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::SpellKind { spell },
                    ParserNode::LexerToken(Token::VhyToSortLater(intermediates::VhyToSortLater::Spell {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::SpecifiedSpell {
                    spell: object::specified_object::SpecifiedSpell {
                        kind: spell.clone(),
                        specifiers: None,
                        #[cfg(feature = "spanned_tree")]
                        span: spell.node_span().merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
