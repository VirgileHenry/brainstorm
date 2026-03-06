mod object_gains_abilities_rules;

pub fn rules() -> impl Iterator<Item = crate::parser::rules::ParserRule> {
    [object_gains_abilities_rules::rules()].into_iter().flatten()
}
