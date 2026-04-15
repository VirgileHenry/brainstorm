use crate::ability_tree::ability::Ability;
use crate::ability_tree::ability::AbilityKind;
use crate::ability_tree::ability::activated::ActivatedAbility;
use crate::ability_tree::cost::Cost;
use crate::ability_tree::imperative::AddManaImperative;
use crate::ability_tree::imperative::Imperative;
use crate::ability_tree::imperative::ImperativeList;
use crate::ability_tree::imperative::ManaToAdd;
use crate::ability_tree::imperative::ManaToAddKind;
use crate::ability_tree::imperative::SacrificeImperative;
use crate::ability_tree::number::FixedNumber;
use crate::ability_tree::number::Number;
use crate::ability_tree::object::ObjectReference;
use crate::ability_tree::object::SelfReferencingObject;
use crate::ability_tree::statement::Statement;
use crate::utils::HeapArrayVec;

pub fn treasure_token_ability() -> crate::AbilityTree {
    crate::AbilityTree {
        abilities: {
            let mut abilities = HeapArrayVec::new();
            let treasure_ability = AbilityKind::Written(Ability::Activated(ActivatedAbility {
                effect: super::spell::SpellAbility {
                    effects: {
                        let mut effects = HeapArrayVec::new();
                        let add_mana_effect = Statement::Imperatives(ImperativeList {
                            executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                                #[cfg(feature = "spanned_tree")]
                                span: Default::default(),
                            },
                            condition: None,
                            imperatives: {
                                let mut imperatives = HeapArrayVec::new();
                                let add_mana_imperative = Imperative::AddMana(AddManaImperative {
                                    mana: {
                                        let mut added_mana = arrayvec::ArrayVec::new();
                                        let mana = ManaToAdd {
                                            kind: ManaToAddKind::AnyColor {
                                                #[cfg(feature = "spanned_tree")]
                                                span: Default::default(),
                                            },
                                            amount: Number::Number(FixedNumber {
                                                number: 1,
                                                #[cfg(feature = "spanned_tree")]
                                                span: Default::default(),
                                            }),
                                            #[cfg(feature = "spanned_tree")]
                                            span: Default::default(),
                                        };
                                        added_mana.push(mana);
                                        added_mana
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: Default::default(),
                                });

                                imperatives.push(add_mana_imperative);
                                imperatives
                            },
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        });

                        effects.push(add_mana_effect);
                        effects
                    },
                    #[cfg(feature = "spanned_tree")]
                    span: Default::default(),
                },
                costs: {
                    let mut costs = HeapArrayVec::new();
                    let sacrifice_self_cost = Cost::Imperative(Imperative::Sacrifice(SacrificeImperative {
                        object: ObjectReference::SelfReferencing(SelfReferencingObject {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }),
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    }));

                    costs.push(sacrifice_self_cost);
                    costs
                },
                #[cfg(feature = "spanned_tree")]
                span: Default::default(),
            }));
            abilities.push(treasure_ability);
            abilities
        },
        #[cfg(feature = "spanned_tree")]
        span: Default::default(),
    }
}
