use crate::ability_tree::ability::statik::continuous_effect::*;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::intermediates;
use crate::parser::rules::ParserNode;
use crate::parser::rules::ParserRule;
use crate::parser::rules::ParserRuleDeclarationLocation;
use crate::parser::rules::RuleLhs;
use crate::utils::dummy;
use idris::Idris;

#[cfg(feature = "spanned_tree")]
use crate::ability_tree::AbilityTreeNode;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    /* "<object> get <power toughness modifiers>" */
    [ParserRule {
        expanded: RuleLhs::new(&[
            ParserNode::Creature { creature: dummy() }.id(),
            ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get {
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }))
            .id(),
            ParserNode::PowerToughnessModifiers { modifiers: dummy() }.id(),
        ]),
        merged: ParserNode::ContinuousEffect { effect: dummy() }.id(),
        reduction: |nodes: &[ParserNode]| match &nodes {
            &[
                ParserNode::Creature { creature },
                ParserNode::LexerToken(Token::ActionKeyword(intermediates::ActionKeyword::Get { .. })),
                ParserNode::PowerToughnessModifiers { modifiers },
            ] => Ok(ParserNode::ContinuousEffect {
                effect: ContinuousEffect {
                    effect: ContinuousEffectKind::ModifyObjectAbilities(ModifyObjectEffect {
                        object: creature.to_permanent(),
                        modifications: {
                            let mut modifications = crate::utils::HeapArrayVec::new();
                            let characteristic_mod = ObjectAbilitiesModification::CharacteristicModification(
                                ObjectCharacteristicModification::PowerToughnessModifiers(modifiers.clone()),
                            );
                            modifications.push(characteristic_mod);
                            modifications
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: creature.node_span().merge(&modifiers.node_span()),
                    }),
                    #[cfg(feature = "spanned_tree")]
                    span: creature.node_span().merge(&modifiers.node_span()),
                },
            }),
            _ => Err("Provided tokens do not match rule definition"),
        },
        creation_loc: ParserRuleDeclarationLocation::here(),
    }]
    .into_iter()
}
