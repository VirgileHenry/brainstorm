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
    /* "another <specified creature>" is a + other creature */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::SpecifiedCreature { creature: dummy() }.id(),
        ]),
        merged: ParserNode::Creature { creature: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::EnglishKeyword(intermediates::EnglishKeyword::Another {
                    #[cfg(feature = "spanned_tree")]
                        span: another_span,
                })),
                ParserNode::SpecifiedCreature { creature },
            ] => Ok(ParserNode::Creature {
                creature: object::Creature::Reference(object::reference::CreatureReference {
                    count: object::CountSpecifier::A {
                        #[cfg(feature = "spanned_tree")]
                        span: *another_span,
                    },
                    creature: creature.add_factor_specifier(object::specified_object::CreatureSpecifier::Another(
                        object::specified_object::AnotherObjectSpecifier {
                            #[cfg(feature = "spanned_tree")]
                            span: *another_span,
                        },
                    )),
                    #[cfg(feature = "spanned_tree")]
                    span: creature.node_span().merge(another_span),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
