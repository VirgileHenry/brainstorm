#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DamageKind {
    Damage,
    CombatDamage,
    NoncombatDamage,
}

impl DamageKind {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "damage" | "damages" => Some(DamageKind::Damage),
            "combat damage" => Some(DamageKind::CombatDamage),
            "noncombat damage" => Some(DamageKind::NoncombatDamage),
            _ => None,
        }
    }
}
