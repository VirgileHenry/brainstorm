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
        /* "<spell kind>" makes a specified spell  */
        ParserRule {
            expanded: RuleLhs::new(&[ParserNode::SpellKind {
                spell: Default::default(),
            }
            .id()]),
            merged: ParserNode::SpecifiedSpell {
                spell: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[ParserNode::SpellKind { spell }] => Ok(ParserNode::SpecifiedSpell {
                    spell: object::specified_object::SpecifiedSpell {
                        kind: spell.clone(),
                        specifiers: None,
                        #[cfg(feature = "spanned_tree")]
                        span: spell.span(),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
        /* "<spell kind> spell" makes a specified spell  */
        ParserRule {
            expanded: RuleLhs::new(&[
                ParserNode::SpellKind {
                    spell: Default::default(),
                }
                .id(),
                ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Spell {
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                }))
                .id(),
            ]),
            merged: ParserNode::SpecifiedSpell {
                spell: Default::default(),
            }
            .id(),
            reduction: |nodes: &[ParserNode]| match &nodes {
                &[
                    ParserNode::SpellKind { spell },
                    ParserNode::LexerToken(Token::GameTerm(intermediate::GameTerm::Spell {
                        #[cfg(feature = "spanned_tree")]
                            span: end_span,
                    })),
                ] => Ok(ParserNode::SpecifiedSpell {
                    spell: object::specified_object::SpecifiedSpell {
                        kind: spell.clone(),
                        specifiers: None,
                        #[cfg(feature = "spanned_tree")]
                        span: spell.span().merge(end_span),
                    },
                }),
                _ => Err("Provided tokens do not match rule definition"),
            },
            creation_loc: ParserRuleDeclarationLocation::here(),
        },
    ]
    .into_iter()
}
