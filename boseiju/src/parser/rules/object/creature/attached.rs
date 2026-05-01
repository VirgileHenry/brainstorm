use crate::ability_tree::object;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "enchanted creature" and "equipped creature" are attached object creature */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[ParserNode::LexerToken(Token::AttachedObject(
            intermediates::AttachedObject::AttachedCreature {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            },
        ))
        .id()]),
        merged: ParserNode::Creature { creature: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::AttachedObject(intermediates::AttachedObject::AttachedCreature {
                    #[cfg(feature = "spanned_tree")]
                    span,
                })),
            ] => Ok(ParserNode::Creature {
                creature: object::Creature::Attached(object::AttachedObject {
                    #[cfg(feature = "spanned_tree")]
                    span: *span,
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
