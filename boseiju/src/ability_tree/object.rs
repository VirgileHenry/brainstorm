#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    CardType(mtg_data::CardType),
    SorcerySubtype(mtg_data::SpellType),
}

impl ObjectKind {
    pub const COUNT: usize = {
        mtg_data::ArtifactType::COUNT
            + mtg_data::BattleType::COUNT
            + 1
            + mtg_data::CreatureType::COUNT
            + mtg_data::EnchantmentType::COUNT
            + mtg_data::SpellType::COUNT
            + mtg_data::LandType::COUNT
            + 1
            + mtg_data::PlaneswalkerType::COUNT
            + 1
            + mtg_data::Supertype::COUNT
            + mtg_data::CardType::COUNT
            + mtg_data::SpellType::COUNT
    };
    pub const fn id(&self) -> usize {
        /* Hey GRT, please don't judge me too much for this */
        match self {
            Self::ArtifactSubtype(subtype) => subtype.id(),
            Self::BattleSubtype(subtype) => mtg_data::ArtifactType::COUNT + subtype.id(),
            Self::Card => mtg_data::ArtifactType::COUNT + mtg_data::BattleType::COUNT,
            Self::CreatureSubtype(subtype) => mtg_data::ArtifactType::COUNT + mtg_data::BattleType::COUNT + 1 + subtype.id(),
            Self::EnchantmentSubtype(subtype) => {
                mtg_data::ArtifactType::COUNT + mtg_data::BattleType::COUNT + 1 + mtg_data::CreatureType::COUNT + subtype.id()
            }
            Self::InstantSubtype(subtype) => {
                mtg_data::ArtifactType::COUNT
                    + mtg_data::BattleType::COUNT
                    + 1
                    + mtg_data::CreatureType::COUNT
                    + mtg_data::EnchantmentType::COUNT
                    + subtype.id()
            }
            Self::LandSubtype(subtype) => {
                mtg_data::ArtifactType::COUNT
                    + mtg_data::BattleType::COUNT
                    + 1
                    + mtg_data::CreatureType::COUNT
                    + mtg_data::EnchantmentType::COUNT
                    + mtg_data::SpellType::COUNT
                    + subtype.id()
            }
            Self::Permanent => {
                mtg_data::ArtifactType::COUNT
                    + mtg_data::BattleType::COUNT
                    + 1
                    + mtg_data::CreatureType::COUNT
                    + mtg_data::EnchantmentType::COUNT
                    + mtg_data::SpellType::COUNT
                    + mtg_data::LandType::COUNT
                    + 1
            }
            Self::PlaneswalkerSubtype(subtype) => {
                mtg_data::ArtifactType::COUNT
                    + mtg_data::BattleType::COUNT
                    + 1
                    + mtg_data::CreatureType::COUNT
                    + mtg_data::EnchantmentType::COUNT
                    + mtg_data::SpellType::COUNT
                    + mtg_data::LandType::COUNT
                    + 1
                    + subtype.id()
            }
            Self::Spell => {
                mtg_data::ArtifactType::COUNT
                    + mtg_data::BattleType::COUNT
                    + 1
                    + mtg_data::CreatureType::COUNT
                    + mtg_data::EnchantmentType::COUNT
                    + mtg_data::SpellType::COUNT
                    + mtg_data::LandType::COUNT
                    + 1
                    + mtg_data::PlaneswalkerType::COUNT
                    + 1
            }
            Self::Supertype(subtype) => {
                mtg_data::ArtifactType::COUNT
                    + mtg_data::BattleType::COUNT
                    + 1
                    + mtg_data::CreatureType::COUNT
                    + mtg_data::EnchantmentType::COUNT
                    + mtg_data::SpellType::COUNT
                    + mtg_data::LandType::COUNT
                    + 1
                    + mtg_data::PlaneswalkerType::COUNT
                    + 1
                    + subtype.id()
            }
            Self::CardType(card_type) => {
                mtg_data::ArtifactType::COUNT
                    + mtg_data::BattleType::COUNT
                    + 1
                    + mtg_data::CreatureType::COUNT
                    + mtg_data::EnchantmentType::COUNT
                    + mtg_data::SpellType::COUNT
                    + mtg_data::LandType::COUNT
                    + 1
                    + mtg_data::PlaneswalkerType::COUNT
                    + 1
                    + mtg_data::Supertype::COUNT
                    + card_type.id()
            }
            Self::SorcerySubtype(subtype) => {
                mtg_data::ArtifactType::COUNT
                    + mtg_data::BattleType::COUNT
                    + 1
                    + mtg_data::CreatureType::COUNT
                    + mtg_data::EnchantmentType::COUNT
                    + mtg_data::SpellType::COUNT
                    + mtg_data::LandType::COUNT
                    + 1
                    + mtg_data::PlaneswalkerType::COUNT
                    + 1
                    + mtg_data::Supertype::COUNT
                    + mtg_data::CardType::COUNT
                    + subtype.id()
            }
        }
    }
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
}

impl crate::ability_tree::AbilityTreeImpl for ObjectReference {
    fn display<W: std::io::Write>(&self, out: &mut crate::utils::TreeFormatter<'_, W>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            ObjectReference::SelfReferencing => write!(out, "Self Referencing (~)"),
            ObjectReference::SpecifiedObj { amount, specifiers } => {
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
