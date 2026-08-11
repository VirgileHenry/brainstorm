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
    /* "another <specified creature>" is a + other creature */
    std::iter::once(ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::SpecifiedCreature {
                creature: Default::default(),
            }
            .id(),
        ]),
        merged: ParserNode::Creature {
            creature: Default::default(),
        }
        .id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::LexerToken(Token::EnglishDeterminer(intermediate::EnglishDeterminer::Another {
                    #[cfg(feature = "spanned_tree")]
                        span: another_span,
                })),
                ParserNode::SpecifiedCreature { creature },
            ] => Ok(ParserNode::Creature {
                creature: object::Creature::Reference(object::reference::CreatureReference {
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
                    creature: creature.add_factor_specifier(object::specified_object::CreatureSpecifier::Another(
                        object::specified_object::AnotherObjectSpecifier {
                            #[cfg(feature = "spanned_tree")]
                            span: *another_span,
                        },
                    )),
                    #[cfg(feature = "spanned_tree")]
                    span: creature.span().merge(another_span),
                }),
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    })
}
