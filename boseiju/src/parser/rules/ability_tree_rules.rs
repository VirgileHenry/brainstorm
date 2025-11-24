use super::ParserNode;
use super::ParserRule;

/* [ParserNode::Ability(ability)] -> ParserNode::AbilityTree(AbilityTree { abilities: vec![ability] }), */

const ABILITY_TO_AB_TREE: ParserRule = ParserRule::create(crate::state_id!([ParserNode::Ability(ability)]), |tokens| {
    let ability = match tokens {
        [ParserNode::Ability(ability)] => *ability.clone(),
        _ => panic!("Invalid tokens for rule!"),
    };
    let mut abilities = arrayvec::ArrayVec::new();
    abilities.push(ability);
    ParserNode::AbilityTree(Box::new(crate::AbilityTree { abilities }))
});
