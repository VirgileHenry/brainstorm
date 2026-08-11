pub trait DeedForm: std::fmt::Debug + Clone + Eq + PartialEq {
    type Quantifier: crate::ability_tree::quantifier::Quantifier + serde::Serialize + for<'de> serde::Deserialize<'de>;
    const FORM_NAME: &'static str;
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveForm;

impl DeedForm for ActiveForm {
    type Quantifier = crate::ability_tree::quantifier::ActiveQuantifier;
    const FORM_NAME: &'static str = "active form";
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassiveForm;

impl DeedForm for PassiveForm {
    type Quantifier = crate::ability_tree::quantifier::PassiveQuantifier;
    const FORM_NAME: &'static str = "passive form";
}
