use crate::ability_tree::ability::Ability;
use crate::ability_tree::ability::WrittenAbility;
use crate::ability_tree::ability::activated::ActivatedAbility;
use crate::ability_tree::cost::Cost;
use crate::ability_tree::imperative::AddManaImperative;
use crate::ability_tree::imperative::Imperative;
use crate::ability_tree::imperative::ImperativeKind;
use crate::ability_tree::imperative::ManaToAdd;
use crate::ability_tree::imperative::ManaToAddOfAnyColor;
use crate::ability_tree::imperative::SacrificeImperative;
use crate::ability_tree::imperative_list::ImperativeList;
use crate::ability_tree::number::FixedNumber;
use crate::ability_tree::number::Number;
use crate::ability_tree::object::Permanent;
use crate::ability_tree::object::SelfReferencing;
use crate::ability_tree::statement::Statement;
use crate::utils::HeapArrayVec;

pub fn treasure_token_ability() -> crate::AbilityTree {
    crate::AbilityTree {
        abilities: {
            let mut abilities = HeapArrayVec::new();
            let treasure_ability = Ability::Written(WrittenAbility::Activated(ActivatedAbility {
                effect: super::spell::SpellAbility {
                    effects: {
                        let mut effects = HeapArrayVec::new();
                        let add_mana_effect = Statement::Imperatives(ImperativeList {
                            imperatives: {
                                let mut imperatives = HeapArrayVec::new();
                                let add_mana_imperative = Imperative {
                                    kind: ImperativeKind::AddMana(AddManaImperative {
                                        possibilities: {
                                            let mut added_mana = crate::utils::HeapArrayVec::new();
                                            let mana = ManaToAdd::AnyColor(ManaToAddOfAnyColor {
                                                amount: Number::Number(FixedNumber {
                                                    number: 1,
                                                    #[cfg(feature = "spanned_tree")]
                                                    span: Default::default(),
                                                }),
                                                #[cfg(feature = "spanned_tree")]
                                                span: Default::default(),
                                            });
                                            added_mana.push(mana);
                                            added_mana
                                        },
                                        #[cfg(feature = "spanned_tree")]
                                        span: Default::default(),
                                    }),
                                    executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                                        #[cfg(feature = "spanned_tree")]
                                        span: Default::default(),
                                    },
                                    #[cfg(feature = "spanned_tree")]
                                    span: Default::default(),
                                };
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
                    let sacrifice_self_cost = Cost::Imperative(Imperative {
                        kind: ImperativeKind::Sacrifice(SacrificeImperative {
                            object: Permanent::SelfReferencing(SelfReferencing {
                                #[cfg(feature = "spanned_tree")]
                                span: Default::default(),
                            }),
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        }),
                        executing_player: crate::ability_tree::player::PlayerSpecifier::You {
                            #[cfg(feature = "spanned_tree")]
                            span: Default::default(),
                        },
                        #[cfg(feature = "spanned_tree")]
                        span: Default::default(),
                    });

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
