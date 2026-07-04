use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedCard {
    AdvocateOfTheBeast {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BasriDevotedPaladin {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Breathstealer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BridesGown {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChandraFlamesCatalyst {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    CrownOfEmpires {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    EightAndAHalfTails {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    EyeOfVecna {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FeralShadow {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FesteringNewt {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Godsire {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HandOfVecna {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HelmOfKaldra {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    JiangYanggu {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LilianaDeathMage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MidnightClock {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MidnightScavenger {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MineWorker {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MuYanling {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MuYanlingCelestialWind {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NissaNaturesArtisan {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PeerThroughDepths {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PlantWorker {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ReachThroughMists {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ScepterOfEmpires {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ShieldOfKaldra {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SunlitHoplit {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SwordOfKaldra {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheUnspeakable {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ThroneOfEmpires {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TowerWorker {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Wastes {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for NamedCard {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::NamedCardIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::NamedCard(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "named choice"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::AdvocateOfTheBeast { span } => *span,
            Self::BasriDevotedPaladin { span } => *span,
            Self::Breathstealer { span } => *span,
            Self::BridesGown { span } => *span,
            Self::ChandraFlamesCatalyst { span } => *span,
            Self::CrownOfEmpires { span } => *span,
            Self::EightAndAHalfTails { span } => *span,
            Self::EyeOfVecna { span } => *span,
            Self::FeralShadow { span } => *span,
            Self::FesteringNewt { span } => *span,
            Self::Godsire { span } => *span,
            Self::HandOfVecna { span } => *span,
            Self::HelmOfKaldra { span } => *span,
            Self::JiangYanggu { span } => *span,
            Self::LilianaDeathMage { span } => *span,
            Self::MidnightClock { span } => *span,
            Self::MidnightScavenger { span } => *span,
            Self::MineWorker { span } => *span,
            Self::MuYanling { span } => *span,
            Self::MuYanlingCelestialWind { span } => *span,
            Self::NissaNaturesArtisan { span } => *span,
            Self::PeerThroughDepths { span } => *span,
            Self::PlantWorker { span } => *span,
            Self::ReachThroughMists { span } => *span,
            Self::ScepterOfEmpires { span } => *span,
            Self::ShieldOfKaldra { span } => *span,
            Self::SunlitHoplit { span } => *span,
            Self::SwordOfKaldra { span } => *span,
            Self::TheUnspeakable { span } => *span,
            Self::ThroneOfEmpires { span } => *span,
            Self::TowerWorker { span } => *span,
            Self::Wastes { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AdvocateOfTheBeast { .. } => write!(f, "accumulated knowledge"),
            Self::BasriDevotedPaladin { .. } => write!(f, "accumulated knowledge"),
            Self::Breathstealer { .. } => write!(f, "accumulated knowledge"),
            Self::BridesGown { .. } => write!(f, "accumulated knowledge"),
            Self::ChandraFlamesCatalyst { .. } => write!(f, "accumulated knowledge"),
            Self::CrownOfEmpires { .. } => write!(f, "accumulated knowledge"),
            Self::EightAndAHalfTails { .. } => write!(f, "accumulated knowledge"),
            Self::EyeOfVecna { .. } => write!(f, "accumulated knowledge"),
            Self::FeralShadow { .. } => write!(f, "accumulated knowledge"),
            Self::FesteringNewt { .. } => write!(f, "accumulated knowledge"),
            Self::Godsire { .. } => write!(f, "accumulated knowledge"),
            Self::HandOfVecna { .. } => write!(f, "accumulated knowledge"),
            Self::HelmOfKaldra { .. } => write!(f, "accumulated knowledge"),
            Self::JiangYanggu { .. } => write!(f, "accumulated knowledge"),
            Self::LilianaDeathMage { .. } => write!(f, "accumulated knowledge"),
            Self::MidnightClock { .. } => write!(f, "accumulated knowledge"),
            Self::MidnightScavenger { .. } => write!(f, "accumulated knowledge"),
            Self::MineWorker { .. } => write!(f, "accumulated knowledge"),
            Self::MuYanling { .. } => write!(f, "accumulated knowledge"),
            Self::MuYanlingCelestialWind { .. } => write!(f, "accumulated knowledge"),
            Self::NissaNaturesArtisan { .. } => write!(f, "accumulated knowledge"),
            Self::PeerThroughDepths { .. } => write!(f, "accumulated knowledge"),
            Self::PlantWorker { .. } => write!(f, "accumulated knowledge"),
            Self::ReachThroughMists { .. } => write!(f, "accumulated knowledge"),
            Self::ScepterOfEmpires { .. } => write!(f, "accumulated knowledge"),
            Self::ShieldOfKaldra { .. } => write!(f, "accumulated knowledge"),
            Self::SunlitHoplit { .. } => write!(f, "accumulated knowledge"),
            Self::SwordOfKaldra { .. } => write!(f, "accumulated knowledge"),
            Self::TheUnspeakable { .. } => write!(f, "accumulated knowledge"),
            Self::ThroneOfEmpires { .. } => write!(f, "accumulated knowledge"),
            Self::TowerWorker { .. } => write!(f, "accumulated knowledge"),
            Self::Wastes { .. } => write!(f, "accumulated knowledge"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedCard {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "advocate of the beast" => Some(Self::AdvocateOfTheBeast {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "basri, devoted paladin" => Some(Self::BasriDevotedPaladin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "breathstealer" => Some(Self::Breathstealer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bride's gown" => Some(Self::BridesGown {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chandra, flame's catalyst" => Some(Self::ChandraFlamesCatalyst {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "crown of empires" => Some(Self::CrownOfEmpires {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "eight-and-a-half-tails" => Some(Self::EightAndAHalfTails {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "feral shadow" => Some(Self::FeralShadow {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "festering newt" => Some(Self::FesteringNewt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "godsire" => Some(Self::Godsire {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "eye of vecna" => Some(Self::EyeOfVecna {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "hand of vecna" => Some(Self::HandOfVecna {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "helm of kaldra" => Some(Self::HelmOfKaldra {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "jiang yanggu" => Some(Self::JiangYanggu {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "liliana, death mage" => Some(Self::LilianaDeathMage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "midnight clock" => Some(Self::MidnightClock {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "midnight scavenger" => Some(Self::MidnightScavenger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mine worker" => Some(Self::MineWorker {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mu yanling" => Some(Self::MuYanling {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mu yanling, celestial wind" => Some(Self::MuYanlingCelestialWind {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nissa, nature's artisan" => Some(Self::NissaNaturesArtisan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "peer through depths" => Some(Self::PeerThroughDepths {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "plant worker" => Some(Self::PlantWorker {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "reach through mists" => Some(Self::ReachThroughMists {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "scepter of empires" => Some(Self::ScepterOfEmpires {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "shield of kaldra" => Some(Self::ShieldOfKaldra {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sunlit hoplit" => Some(Self::SunlitHoplit {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sword of kaldra" => Some(Self::SwordOfKaldra {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the unspeakable" => Some(Self::TheUnspeakable {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "throne of empires" => Some(Self::ThroneOfEmpires {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tower worker" => Some(Self::TowerWorker {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "wastes" => Some(Self::Wastes {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
