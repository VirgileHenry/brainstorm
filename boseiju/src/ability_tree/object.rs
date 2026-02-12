#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ObjectKind {
    ArtifactSubtype(mtg_data::ArtifactType),
    BattleSubtype(mtg_data::BattleType),
    Card,
    CreatureSubtype(mtg_data::CreatureType),
    EnchantmentSubtype(mtg_data::EnchantmentType),
    InstantSorcerySubtype(mtg_data::SpellType),
    LandSubtype(mtg_data::LandType),
    Permanent,
    PlaneswalkerSubtype(mtg_data::PlaneswalkerType),
    Spell,
    Supertype(mtg_data::Supertype),
    CardType(mtg_data::CardType),
}

impl std::fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectKind::ArtifactSubtype(subtype) => subtype.fmt(f),
            ObjectKind::BattleSubtype(subtype) => subtype.fmt(f),
            ObjectKind::Card => write!(f, "card"),
            ObjectKind::CreatureSubtype(subtype) => subtype.fmt(f),
            ObjectKind::EnchantmentSubtype(subtype) => subtype.fmt(f),
            ObjectKind::InstantSorcerySubtype(subtype) => subtype.fmt(f),
            ObjectKind::LandSubtype(subtype) => subtype.fmt(f),
            ObjectKind::Permanent => write!(f, "permanent"),
            ObjectKind::PlaneswalkerSubtype(subtype) => subtype.fmt(f),
            ObjectKind::Spell => write!(f, "spell"),
            ObjectKind::Supertype(supertype) => supertype.fmt(f),
            ObjectKind::CardType(ty) => ty.fmt(f),
        }
    }
}

impl crate::ability_tree::terminals::Terminal for ObjectKind {
    fn try_from_str(source: &str) -> Option<Self> {
        if let Some(subtype) = mtg_data::ArtifactType::try_from_str(source) {
            return Some(Self::ArtifactSubtype(subtype));
        } else if let Some(subtype) = mtg_data::BattleType::try_from_str(source) {
            return Some(Self::BattleSubtype(subtype));
        } else if let Some(subtype) = mtg_data::CreatureType::try_from_str(source) {
            return Some(Self::CreatureSubtype(subtype));
        } else if let Some(subtype) = mtg_data::EnchantmentType::try_from_str(source) {
            return Some(Self::EnchantmentSubtype(subtype));
        } else if let Some(subtype) = mtg_data::SpellType::try_from_str(source) {
            return Some(Self::InstantSorcerySubtype(subtype));
        } else if let Some(subtype) = mtg_data::LandType::try_from_str(source) {
            return Some(Self::LandSubtype(subtype));
        } else if let Some(subtype) = mtg_data::PlaneswalkerType::try_from_str(source) {
            return Some(Self::PlaneswalkerSubtype(subtype));
        } else if let Some(subtype) = mtg_data::Supertype::try_from_str(source) {
            return Some(Self::Supertype(subtype));
        } else if let Some(subtype) = mtg_data::CardType::try_from_str(source) {
            return Some(Self::CardType(subtype));
        } else {
            match source {
                "card" | "cards" => Some(Self::Card),
                "permanent" | "permanents" => Some(Self::Permanent),
                "spell" | "spells" => Some(Self::Spell),
                _ => None,
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ObjectReference {
    SelfReferencing,
    SpecifiedObj {
        amount: crate::ability_tree::terminals::CountSpecifier,
        specifiers: ObjectSpecifiers,
    },
    ObjectAttachedTo,
}

impl crate::ability_tree::AbilityTreeImpl for ObjectReference {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::SelfReferencing => write!(out, "self referencing (~)"),
            Self::ObjectAttachedTo => write!(out, "object this is attached to"),
            Self::SpecifiedObj { amount, specifiers } => {
                write!(out, "specified object:")?;
                out.push_inter_branch()?;
                write!(out, "amount:")?;
                out.push_final_branch()?;
                write!(out, "{amount}")?;
                out.pop_branch();
                out.next_final_branch()?;
                write!(out, "specifier(s):")?;
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
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ObjectSpecifiers {
    Single(ObjectSpecifier),
    And(arrayvec::ArrayVec<ObjectSpecifier, 16>),
    Or(arrayvec::ArrayVec<ObjectSpecifier, 16>),
    /// This one is a bit tricky, but it avoids recursive specifiers.
    /// We can have crosses of and / or: "basic forest or plain" is "(basic and forest) or (basic and plain)".
    OrOfAnd(arrayvec::ArrayVec<arrayvec::ArrayVec<ObjectSpecifier, 4>, 4>),
}

impl crate::ability_tree::AbilityTreeImpl for ObjectSpecifiers {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ObjectSpecifiers::Single(specifier) => specifier.display(out),
            ObjectSpecifiers::And(specifiers) => {
                write!(out, "and:")?;
                for (i, specifier) in specifiers.iter().enumerate() {
                    if i == specifiers.len() - 1 {
                        out.push_final_branch()?;
                    } else {
                        out.push_inter_branch()?;
                    }
                    specifier.display(out)?;
                    out.pop_branch();
                }
                Ok(())
            }
            ObjectSpecifiers::Or(specifiers) => {
                write!(out, "or:")?;
                for (i, specifier) in specifiers.iter().enumerate() {
                    if i == specifiers.len() - 1 {
                        out.push_final_branch()?;
                    } else {
                        out.push_inter_branch()?;
                    }
                    specifier.display(out)?;
                    out.pop_branch();
                }
                Ok(())
            }
            ObjectSpecifiers::OrOfAnd(specifiers) => {
                write!(out, "or:")?;
                for (i, and_specifiers) in specifiers.iter().enumerate() {
                    if i == specifiers.len() - 1 {
                        out.push_final_branch()?;
                    } else {
                        out.push_inter_branch()?;
                    }
                    write!(out, "and:")?;
                    for (j, specifier) in and_specifiers.iter().enumerate() {
                        if j == and_specifiers.len() - 1 {
                            out.push_final_branch()?;
                        } else {
                            out.push_inter_branch()?;
                        }
                        specifier.display(out)?;
                        out.pop_branch();
                    }
                    out.pop_branch();
                }
                Ok(())
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ObjectSpecifier {
    Color(mtg_data::Color),
    Control(crate::ability_tree::terminals::ControlSpecifier),
    Cast(crate::ability_tree::terminals::CastSpecifier),
    Kind(ObjectKind),
    NotOfAKind(ObjectKind),
}

impl crate::ability_tree::AbilityTreeImpl for ObjectSpecifier {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ObjectSpecifier::Color(color) => write!(out, "color specifier: {color}"),
            ObjectSpecifier::Kind(object) => write!(out, "kind specifier: {object}"),
            ObjectSpecifier::NotOfAKind(object) => write!(out, "not of a kind specifier: not {object}"),
            ObjectSpecifier::Control(control) => write!(out, "control specifier: {control}"),
            ObjectSpecifier::Cast(cast) => write!(out, "cast specifier: {cast}"),
        }
    }
}
