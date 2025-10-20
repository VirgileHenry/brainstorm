#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize
)]
pub enum ObjectKind {
    ArtifactSubtype(mtg_data::ArtifactType),
    BattleSubtype(mtg_data::BattleType),
    Card,
    CreatureSubtype(mtg_data::CreatureType),
    EnchantmentSubtype(mtg_data::EnchantmentType),
    InstantSubtype(mtg_data::SpellType),
    LandSubtype(mtg_data::LandType),
    Permanent,
    PlaneswalkerSubtype(mtg_data::PlaneswalkerType),
    Spell,
    Supertype(mtg_data::Supertype),
    Type(mtg_data::CardType),
    SorcerySubtype(mtg_data::SpellType),
}

impl std::fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectKind::ArtifactSubtype(subtype) => subtype.fmt(f),
            ObjectKind::BattleSubtype(subtype) => subtype.fmt(f),
            ObjectKind::Card => write!(f, "card"),
            ObjectKind::CreatureSubtype(subtype) => subtype.fmt(f),
            ObjectKind::EnchantmentSubtype(subtype) => subtype.fmt(f),
            ObjectKind::InstantSubtype(subtype) => subtype.fmt(f),
            ObjectKind::LandSubtype(subtype) => subtype.fmt(f),
            ObjectKind::Permanent => write!(f, "permanent"),
            ObjectKind::PlaneswalkerSubtype(subtype) => subtype.fmt(f),
            ObjectKind::Spell => write!(f, "spell"),
            ObjectKind::Supertype(supertype) => supertype.fmt(f),
            ObjectKind::Type(ty) => ty.fmt(f),
            ObjectKind::SorcerySubtype(subtype) => subtype.fmt(f),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObjectReference {
    SelfReferencing,
    SpecifiedObj {
        amount: crate::ability_tree::terminals::CountSpecifier,
        specifier: ObjectSpecifiers,
    },
}

impl crate::ability_tree::AbilityTreeImpl for ObjectReference {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ObjectReference::SelfReferencing => write!(out, "Self Referencing (~)"),
            ObjectReference::SpecifiedObj {
                amount,
                specifier: specifiers,
            } => {
                write!(out, "Specified Object:")?;
                out.push_inter_branch()?;
                write!(out, "Amount:")?;
                out.push_final_branch()?;
                write!(out, "{amount}")?;
                out.pop_branch();
                out.next_final_branch()?;
                write!(out, "Specifier(s):")?;
                out.push_final_branch()?;
                specifiers.display(out)?;
                out.pop_branch();
                out.pop_branch();
                Ok(())
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObjectSpecifiers {
    Single(ObjectSpecifier),
    And(arrayvec::ArrayVec<ObjectSpecifier, 8>),
    Or(arrayvec::ArrayVec<ObjectSpecifier, 8>),
}

impl crate::ability_tree::AbilityTreeImpl for ObjectSpecifiers {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ObjectSpecifiers::Single(specifier) => specifier.display(out),
            ObjectSpecifiers::And(specifiers) => {
                for specifier in specifiers.iter().take(specifiers.len().saturating_sub(1)) {
                    specifier.display(out)?;
                    write!(out, " and ")?;
                }
                if let Some(specifier) = specifiers.last() {
                    specifier.display(out)?;
                }
                Ok(())
            }
            ObjectSpecifiers::Or(specifiers) => {
                for specifier in specifiers.iter().take(specifiers.len().saturating_sub(1)) {
                    specifier.display(out)?;
                    write!(out, " or ")?;
                }
                if let Some(specifier) = specifiers.last() {
                    specifier.display(out)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObjectSpecifier {
    Color(mtg_data::Color),
    Control(crate::ability_tree::terminals::ControlSpecifier),
    Kind(ObjectKind),
}

impl crate::ability_tree::AbilityTreeImpl for ObjectSpecifier {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ObjectSpecifier::Color(color) => write!(out, "color specifier: {color}"),
            ObjectSpecifier::Kind(object) => write!(out, "kind specifier: {object}"),
            ObjectSpecifier::Control(control) => write!(out, "control specifier: {control}"),
        }
    }
}
