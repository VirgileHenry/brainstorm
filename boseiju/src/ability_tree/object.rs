#[derive(idris::Idris)]
#[idris(repr = u16)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObjectKind {
    #[idris(rec)]
    ArtifactSubtype(mtg_data::ArtifactType),
    #[idris(rec)]
    BattleSubtype(mtg_data::BattleType),
    Card,
    #[idris(rec)]
    CreatureSubtype(mtg_data::CreatureType),
    #[idris(rec)]
    EnchantmentSubtype(mtg_data::EnchantmentType),
    #[idris(rec)]
    InstantSubtype(mtg_data::SpellType),
    #[idris(rec)]
    LandSubtype(mtg_data::LandType),
    Permanent,
    #[idris(rec)]
    PlaneswalkerSubtype(mtg_data::PlaneswalkerType),
    Spell,
    #[idris(rec)]
    Supertype(mtg_data::Supertype),
    #[idris(rec)]
    CardType(mtg_data::CardType),
    #[idris(rec)]
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
            ObjectKind::CardType(ty) => ty.fmt(f),
            ObjectKind::SorcerySubtype(subtype) => subtype.fmt(f),
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
            return Some(Self::InstantSubtype(subtype));
        } else if let Some(subtype) = mtg_data::LandType::try_from_str(source) {
            return Some(Self::LandSubtype(subtype));
        } else if let Some(subtype) = mtg_data::PlaneswalkerType::try_from_str(source) {
            return Some(Self::PlaneswalkerSubtype(subtype));
        } else if let Some(subtype) = mtg_data::Supertype::try_from_str(source) {
            return Some(Self::Supertype(subtype));
        } else if let Some(subtype) = mtg_data::CardType::try_from_str(source) {
            return Some(Self::CardType(subtype));
        } else if let Some(subtype) = mtg_data::SpellType::try_from_str(source) {
            return Some(Self::SorcerySubtype(subtype));
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
        }
    }
}
