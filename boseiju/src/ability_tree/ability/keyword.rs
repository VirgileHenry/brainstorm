mod keyword_to_abilities;
mod standalone_kw_abilities;

pub use keyword_to_abilities::keyword_to_abilities;
pub use standalone_kw_abilities::all_standalone_kw_abilities;

use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;

/// This is basically a 1-1 copy of the [`mtg_data::KeywordAbility`],
/// expect all keyword abilities required additional text also have this text.
///
/// For instance, "Ward" on its own isn't truly a keyword abilty: It's "ward: pay 2 life".
#[derive(idris_derive::Idris)]
#[idris(repr = usize)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub enum ExpandedKeywordAbility {
    Absorb,
    Affinity,
    Afflict,
    Afterlife,
    Aftermath,
    Amplify,
    Annihilator,
    Ascend,
    Assist,
    Augment,
    AuraSwap,
    Awaken,
    Backup,
    Banding,
    Bargain,
    BasicLandcycling,
    BattleCry,
    Bestow,
    Blitz,
    Bloodthirst,
    Boast,
    Bushido,
    Buyback,
    Cascade,
    Casualty,
    Champion,
    Changeling,
    ChooseABackground,
    Cipher,
    Cleave,
    CommanderNinjutsu,
    Companion,
    Compleated,
    Conspire,
    Convoke,
    Craft,
    Crew,
    CumulativeUpkeep,
    Cycling,
    Dash,
    Daybound,
    Deathtouch,
    Decayed,
    Defender,
    Delve,
    Demonstrate,
    Desertwalk,
    Dethrone,
    Devoid,
    Devour,
    Disguise,
    Disturb,
    DoctorsCompanion,
    DoubleAgenda,
    DoubleStrike,
    DoubleTeam,
    Dredge,
    Echo,
    Embalm,
    Emerge,
    Enchant,
    Encore,
    Enlist,
    Entwine,
    Epic,
    Equip,
    Escalate,
    Escape,
    Eternalize,
    Evoke,
    Evolve,
    Exalted,
    Exhaust,
    Exploit,
    Extort,
    Fabricate,
    Fading,
    Fear,
    FirstStrike,
    Flanking,
    Flash,
    Flashback,
    Flying,
    ForMirrodin,
    Forecast,
    Forestcycling,
    Forestwalk,
    Foretell,
    Fortify,
    Freerunning,
    Frenzy,
    FriendsForever,
    Fuse,
    Gift,
    Graft,
    Gravestorm,
    Harmonize,
    Haste,
    Haunt,
    Hexproof,
    HexproofFrom,
    HiddenAgenda,
    Hideaway,
    Horsemanship,
    Impending,
    Improvise,
    Indestructible,
    Infect,
    Ingest,
    Intensity,
    Intimidate,
    Islandcycling,
    Islandwalk,
    JobSelect,
    JumpStart,
    Kicker,
    Landcycling,
    Landwalk,
    LegendaryLandwalk,
    LevelUp,
    Lifelink,
    LivingMetal,
    LivingWeapon,
    Madness,
    MaxSpeed,
    Mayhem,
    Megamorph,
    Melee,
    Menace,
    Mentor,
    Miracle,
    Mobilize,
    Modular,
    MoreThanMeetsTheEye,
    Morph,
    Mountaincycling,
    Mountainwalk,
    Multikicker,
    Mutate,
    Myriad,
    Nightbound,
    Ninjutsu,
    NonbasicLandwalk,
    Offering,
    Offspring,
    Outlast,
    Overload,
    Partner,
    PartnerWith,
    Persist,
    Phasing,
    Plainscycling,
    Plainswalk,
    Poisonous,
    Protection,
    Prototype,
    Provoke,
    Prowess,
    Prowl,
    Rampage,
    Ravenous,
    Reach,
    ReadAhead,
    Rebound,
    Reconfigure,
    Recover,
    Reinforce,
    Renown,
    Replicate,
    Retrace,
    Riot,
    Ripple,
    Saddle,
    Scavenge,
    Shadow,
    Shroud,
    Skulk,
    Slivercycling,
    Soulbond,
    Soulshift,
    Specialize,
    Spectacle,
    Splice,
    SplitSecond,
    Spree,
    Squad,
    Station,
    Storm,
    Sunburst,
    Surge,
    Suspend,
    Swampcycling,
    Swampwalk,
    Toxic,
    Training,
    Trample,
    Transfigure,
    Transmute,
    Tribute,
    Typecycling,
    UmbraArmor,
    Undaunted,
    Undying,
    Unearth,
    Unleash,
    Vanishing,
    Vigilance,
    Ward(WardKeywordAbility),
    Warp,
    WebSlinging,
    Wither,
    Wizardcycling,
}

impl crate::ability_tree::AbilityTreeNode for ExpandedKeywordAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::ExpandedKeywordAbilityIdMarker.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        match self {
            Self::Ward(ward) => children.push(ward as &dyn AbilityTreeNode),
            other => {
                let child_id = crate::ability_tree::NodeKind::ExpandedKeywordAbility(other.clone()).id();
                let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
                children.push(child as &dyn AbilityTreeNode);
            }
        }
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Self::Ward(ward) => ward.display(out)?,
            _ => write!(out, "some keyword ability (fixme)")?,
        }
        Ok(())
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for ExpandedKeywordAbility {
    fn dummy_init() -> Self {
        Self::Absorb
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct WardKeywordAbility {
    cost: crate::ability_tree::cost::Cost,
}

impl crate::ability_tree::AbilityTreeNode for WardKeywordAbility {
    fn node_id(&self) -> usize {
        use idris::Idris;
        crate::ability_tree::NodeKind::WardKeywordAbility.id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        let mut children = arrayvec::ArrayVec::new_const();
        children.push(&self.cost as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "ward—")?;
        self.cost.display(out)?;
        Ok(())
    }
}

impl idris::Idris<usize> for WardKeywordAbility {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for WardKeywordAbility {
    fn dummy_init() -> WardKeywordAbility {
        Self {
            cost: crate::utils::dummy(),
        }
    }
}
