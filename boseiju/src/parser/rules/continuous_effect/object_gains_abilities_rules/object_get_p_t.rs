use crate::ability_tree::ability::statik::continuous_effect::*;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::lexer::tokens::intermediates::PowerToughnessModElements;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "<object> get +n/+n" */
    [ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::ObjectReference { reference: dummy() }.id(),
            ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
            ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::Number { number: dummy() }.id(),
        ]),
        merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::ObjectReference { reference },
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus {
                    #[cfg(feature = "spanned_tree")]
                    span,
                })),
                ParserNode::Number { number: power },
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Bar { .. })),
                ParserNode::LexerToken(Token::PowerToughnessModElements(PowerToughnessModElements::Plus { .. })),
                ParserNode::Number { number: toughness },
            ] => Ok(ParserNode::ContinuousEffect {
                effect: ContinuousEffect {
                    effect: ContinuousEffectKind::ModifyObjectAbilities(ContinuousEffectModifyObject {
                        object: reference.clone(),
                        modifications: {
                            let mut modifications = crate::utils::HeapArrayVec::new();
                            let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                ObjectCharacteristicModification::PowerToughnessModifiers(PowerToughnessModifiers::PlusPlus(
                                    PowerToughnessModifiersPlusPlus {
                                        power_mod: power.clone(),
                                        toughness_mod: toughness.clone(),
                                        #[cfg(feature = "spanned_tree")]
                                        span: span.merge(&toughness.node_span()),
                                    },
                                )),
                            );

                            modifications.push(characteristic_mod);
                            modifications
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: reference.node_span().merge(&toughness.node_span()),
                    }),
                    #[cfg(feature = "spanned_tree")]
                    span: reference.node_span().merge(&toughness.node_span()),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
