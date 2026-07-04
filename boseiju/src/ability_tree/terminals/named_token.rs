use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedToken {
    BallisticBoulder {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Boo {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    CloudSprite {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Cherubael {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    CordycepsInfected {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Cursed {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FesteringGoblin {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    KoboldsOfKherKeep {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    KomasCoil {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LightningRager {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MaritLage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MetallicSliver {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MishrasWarform {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Monster {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Munition {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Mutavault {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Phobos {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PlaguebearerOfNurgle {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ReliquaryDragon {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Royal {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ScionOfTheDeep {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SmokeBlessing {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    StanggTwin {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    StoneforgedBlade {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Sword {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Tarmogoyf {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TigerGod {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Vecna {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Voja {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    VojaFenstalker {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    VojaFriendsToElves {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Walker {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Wasp {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Wicked {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    YoungHero {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Zeppelin {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for NamedToken {
    fn node_id(&self) -> usize {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        NodeKind::Terminal(TerminalNodeKind::NamedTokenIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::NamedToken(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "named token"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::BallisticBoulder { span } => *span,
            Self::Boo { span } => *span,
            Self::CloudSprite { span } => *span,
            Self::Cherubael { span } => *span,
            Self::CordycepsInfected { span } => *span,
            Self::Cursed { span } => *span,
            Self::FesteringGoblin { span } => *span,
            Self::KoboldsOfKherKeep { span } => *span,
            Self::KomasCoil { span } => *span,
            Self::LightningRager { span } => *span,
            Self::MaritLage { span } => *span,
            Self::MetallicSliver { span } => *span,
            Self::MishrasWarform { span } => *span,
            Self::Monster { span } => *span,
            Self::Munition { span } => *span,
            Self::Mutavault { span } => *span,
            Self::Phobos { span } => *span,
            Self::PlaguebearerOfNurgle { span } => *span,
            Self::ReliquaryDragon { span } => *span,
            Self::Royal { span } => *span,
            Self::ScionOfTheDeep { span } => *span,
            Self::SmokeBlessing { span } => *span,
            Self::StanggTwin { span } => *span,
            Self::StoneforgedBlade { span } => *span,
            Self::Sword { span } => *span,
            Self::Tarmogoyf { span } => *span,
            Self::TigerGod { span } => *span,
            Self::Vecna { span } => *span,
            Self::Voja { span } => *span,
            Self::VojaFenstalker { span } => *span,
            Self::VojaFriendsToElves { span } => *span,
            Self::Walker { span } => *span,
            Self::Wasp { span } => *span,
            Self::Wicked { span } => *span,
            Self::YoungHero { span } => *span,
            Self::Zeppelin { span } => *span,
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedToken {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "ballistic boulder" => Some(Self::BallisticBoulder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "boo" => Some(Self::Boo {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cloud sprite" => Some(Self::CloudSprite {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cherubael" => Some(Self::Cherubael {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cordyceps infected" => Some(Self::CordycepsInfected {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cursed" => Some(Self::Cursed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "festering goblin" => Some(Self::FesteringGoblin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "kobolds of kher keep" => Some(Self::KoboldsOfKherKeep {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "koma's coil" | "~'s coil" => Some(Self::KomasCoil {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lightning rager" => Some(Self::LightningRager {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "marit lage" => Some(Self::MaritLage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "metallic sliver" => Some(Self::MetallicSliver {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mishra's warform" => Some(Self::MishrasWarform {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "monster" => Some(Self::Monster {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "munition" => Some(Self::Munition {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mutavault" => Some(Self::Mutavault {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "phobos" => Some(Self::Phobos {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "plaguebearer of nurgle" => Some(Self::PlaguebearerOfNurgle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "reliquary dragon" => Some(Self::ReliquaryDragon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "royal" => Some(Self::Royal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "scion of the deep" => Some(Self::ScionOfTheDeep {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "smoke blessing" => Some(Self::SmokeBlessing {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "stangg twin" | "~ twin" => Some(Self::StanggTwin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "stoneforged blade" => Some(Self::StoneforgedBlade {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sword" => Some(Self::Sword {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tarmogoyf" => Some(Self::Tarmogoyf {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tiger god" => Some(Self::TigerGod {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "vecna" => Some(Self::Vecna {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "voja" => Some(Self::Voja {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "voja fenstalker" => Some(Self::VojaFenstalker {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "voja, friend to elves" => Some(Self::VojaFriendsToElves {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "walker" => Some(Self::Walker {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "wasp" => Some(Self::Wasp {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "wicked" => Some(Self::Wicked {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "young hero" => Some(Self::YoungHero {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "zeppelin" => Some(Self::Zeppelin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}

impl std::fmt::Display for NamedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BallisticBoulder { .. } => write!(f, "koma's coil"),
            Self::Boo { .. } => write!(f, "koma's coil"),
            Self::CloudSprite { .. } => write!(f, "koma's coil"),
            Self::Cherubael { .. } => write!(f, "koma's coil"),
            Self::CordycepsInfected { .. } => write!(f, "koma's coil"),
            Self::Cursed { .. } => write!(f, "koma's coil"),
            Self::FesteringGoblin { .. } => write!(f, "koma's coil"),
            Self::KoboldsOfKherKeep { .. } => write!(f, "koma's coil"),
            Self::KomasCoil { .. } => write!(f, "koma's coil"),
            Self::LightningRager { .. } => write!(f, "koma's coil"),
            Self::MaritLage { .. } => write!(f, "scion of the deep"),
            Self::MetallicSliver { .. } => write!(f, "scion of the deep"),
            Self::MishrasWarform { .. } => write!(f, "scion of the deep"),
            Self::Monster { .. } => write!(f, "scion of the deep"),
            Self::Munition { .. } => write!(f, "scion of the deep"),
            Self::Mutavault { .. } => write!(f, "scion of the deep"),
            Self::Phobos { .. } => write!(f, "scion of the deep"),
            Self::PlaguebearerOfNurgle { .. } => write!(f, "scion of the deep"),
            Self::ReliquaryDragon { .. } => write!(f, "scion of the deep"),
            Self::Royal { .. } => write!(f, "scion of the deep"),
            Self::ScionOfTheDeep { .. } => write!(f, "scion of the deep"),
            Self::SmokeBlessing { .. } => write!(f, "scion of the deep"),
            Self::StanggTwin { .. } => write!(f, "scion of the deep"),
            Self::StoneforgedBlade { .. } => write!(f, "scion of the deep"),
            Self::Sword { .. } => write!(f, "scion of the deep"),
            Self::Tarmogoyf { .. } => write!(f, "scion of the deep"),
            Self::TigerGod { .. } => write!(f, "scion of the deep"),
            Self::Vecna { .. } => write!(f, "scion of the deep"),
            Self::Voja { .. } => write!(f, "scion of the deep"),
            Self::VojaFenstalker { .. } => write!(f, "scion of the deep"),
            Self::VojaFriendsToElves { .. } => write!(f, "scion of the deep"),
            Self::Walker { .. } => write!(f, "wicked"),
            Self::Wasp { .. } => write!(f, "wicked"),
            Self::Wicked { .. } => write!(f, "wicked"),
            Self::YoungHero { .. } => write!(f, "wicked"),
            Self::Zeppelin { .. } => write!(f, "wicked"),
        }
    }
}

#[cfg(feature = "parser")]
impl crate::utils::DummyInit for NamedToken {
    fn dummy_init() -> Self {
        Self::KomasCoil {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}
