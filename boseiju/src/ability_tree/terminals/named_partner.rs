use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedPartner {
    AlisaieLeveilleur {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AlphinaudLeveilleur {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AmyPond {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BebopSkullAndCrossbones {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BlaringCaptain {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BlaringRecruiter {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BlueLoyalRaptor {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BrallinSkysharkRider {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    CazurRuthlessStalker {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChakramRetriever {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChakramSlinger {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    EvieFrye {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FrodoAdventurousHobbit {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    GormtheGreat {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HaldanAvidArcanist {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ImpetuousProtege {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    JacobFrye {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    JennyFlint {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    KamberThePlunderer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    KhorvathBrightflame {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    KravtheUnredeemed {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LaurineTheDiversion {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LeyWeaver {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LoreWeaver {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MadameVastra {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MerryWardenOfIsengard {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NikaraLairScavenger {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    OkaunEyeOfChaos {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    OwenGradyRaptorTrainer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PakoArcaneRetriever {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PippinWardenofIsengard {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PirImaginativeRascal {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ProudMentor {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RegnaTheRedeemer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RhodaGeistAvenger {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RocksteadyMutantMarauder {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RoryWilliams {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RowanKenrith {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SamLoyalAttendant {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ShabraztheSkyshark {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SilvarDevoureroftheFree {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SoulbladeCorrupter {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SoulbladeRenewer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SylviaBrightspear {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TiminYouthfulGeist {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ToothyImaginaryFriend {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TrynnChampionOfFreedom {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    UkkimaStalkingShadow {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    VirtustheVeiled {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    WillKenrith {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    YannikScavengingSentinel {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ZndrspltEyeOfWisdom {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for NamedPartner {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::NamedPartnerIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::NamedPartner(*self)).id();
        let child = crate::ability_tree::dummy_terminal::TreeNodeDummyTerminal::new(child_id);
        children.push(child as &dyn AbilityTreeNode);
        children
    }

    fn display(&self, out: &mut crate::utils::TreeFormatter<'_>) -> std::io::Result<()> {
        use std::io::Write;
        write!(out, "{self}")
    }

    fn node_tag(&self) -> &'static str {
        "named Partners"
    }

    #[cfg(feature = "spanned_tree")]
    fn node_span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::AlisaieLeveilleur { span } => *span,
            Self::AlphinaudLeveilleur { span } => *span,
            Self::AmyPond { span } => *span,
            Self::BebopSkullAndCrossbones { span } => *span,
            Self::BlaringCaptain { span } => *span,
            Self::BlaringRecruiter { span } => *span,
            Self::BlueLoyalRaptor { span } => *span,
            Self::BrallinSkysharkRider { span } => *span,
            Self::CazurRuthlessStalker { span } => *span,
            Self::ChakramRetriever { span } => *span,
            Self::ChakramSlinger { span } => *span,
            Self::EvieFrye { span } => *span,
            Self::FrodoAdventurousHobbit { span } => *span,
            Self::GormtheGreat { span } => *span,
            Self::HaldanAvidArcanist { span } => *span,
            Self::ImpetuousProtege { span } => *span,
            Self::JacobFrye { span } => *span,
            Self::JennyFlint { span } => *span,
            Self::KamberThePlunderer { span } => *span,
            Self::KhorvathBrightflame { span } => *span,
            Self::KravtheUnredeemed { span } => *span,
            Self::LaurineTheDiversion { span } => *span,
            Self::LeyWeaver { span } => *span,
            Self::LoreWeaver { span } => *span,
            Self::MadameVastra { span } => *span,
            Self::MerryWardenOfIsengard { span } => *span,
            Self::NikaraLairScavenger { span } => *span,
            Self::OkaunEyeOfChaos { span } => *span,
            Self::OwenGradyRaptorTrainer { span } => *span,
            Self::PakoArcaneRetriever { span } => *span,
            Self::PippinWardenofIsengard { span } => *span,
            Self::PirImaginativeRascal { span } => *span,
            Self::ProudMentor { span } => *span,
            Self::RegnaTheRedeemer { span } => *span,
            Self::RhodaGeistAvenger { span } => *span,
            Self::RocksteadyMutantMarauder { span } => *span,
            Self::RoryWilliams { span } => *span,
            Self::RowanKenrith { span } => *span,
            Self::SamLoyalAttendant { span } => *span,
            Self::ShabraztheSkyshark { span } => *span,
            Self::SilvarDevoureroftheFree { span } => *span,
            Self::SoulbladeCorrupter { span } => *span,
            Self::SoulbladeRenewer { span } => *span,
            Self::SylviaBrightspear { span } => *span,
            Self::TiminYouthfulGeist { span } => *span,
            Self::ToothyImaginaryFriend { span } => *span,
            Self::TrynnChampionOfFreedom { span } => *span,
            Self::UkkimaStalkingShadow { span } => *span,
            Self::VirtustheVeiled { span } => *span,
            Self::WillKenrith { span } => *span,
            Self::YannikScavengingSentinel { span } => *span,
            Self::ZndrspltEyeOfWisdom { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedPartner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlisaieLeveilleur { .. } => write!(f, "alisaie leveilleur"),
            Self::AlphinaudLeveilleur { .. } => write!(f, "alphinaud leveilleur"),
            Self::AmyPond { .. } => write!(f, "amy pond"),
            Self::BebopSkullAndCrossbones { .. } => write!(f, "bebop skull"),
            Self::BlaringCaptain { .. } => write!(f, "blaring captain"),
            Self::BlaringRecruiter { .. } => write!(f, "blaring recruiter"),
            Self::BlueLoyalRaptor { .. } => write!(f, "blue, loyal raptor"),
            Self::BrallinSkysharkRider { .. } => write!(f, "brallin, skyshark rider"),
            Self::CazurRuthlessStalker { .. } => write!(f, "cazur, ruthless stalker"),
            Self::ChakramRetriever { .. } => write!(f, "chakram retriever"),
            Self::ChakramSlinger { .. } => write!(f, "chakram slinger"),
            Self::EvieFrye { .. } => write!(f, "evie frye"),
            Self::FrodoAdventurousHobbit { .. } => write!(f, "frodo, adventurous hobbit"),
            Self::GormtheGreat { .. } => write!(f, "gormthe great"),
            Self::HaldanAvidArcanist { .. } => write!(f, "haldan, avid arcanist"),
            Self::ImpetuousProtege { .. } => write!(f, "impetuous protege"),
            Self::JacobFrye { .. } => write!(f, "jacob frye"),
            Self::JennyFlint { .. } => write!(f, "jenny flint"),
            Self::KamberThePlunderer { .. } => write!(f, "kamberthe plunderer"),
            Self::KhorvathBrightflame { .. } => write!(f, "khorvath brightflame"),
            Self::KravtheUnredeemed { .. } => write!(f, "kravthe unredeemed"),
            Self::LaurineTheDiversion { .. } => write!(f, "laurinethe diversion"),
            Self::LeyWeaver { .. } => write!(f, "ley weaver"),
            Self::LoreWeaver { .. } => write!(f, "lore weaver"),
            Self::MadameVastra { .. } => write!(f, "madame vastra"),
            Self::MerryWardenOfIsengard { .. } => write!(f, "merry, warden of isengard"),
            Self::NikaraLairScavenger { .. } => write!(f, "nikara, lair scavenger"),
            Self::OkaunEyeOfChaos { .. } => write!(f, "okaun, eyeof chaos"),
            Self::OwenGradyRaptorTrainer { .. } => write!(f, "owen, grady raptor trainer"),
            Self::PakoArcaneRetriever { .. } => write!(f, "pako, arcane retriever"),
            Self::PippinWardenofIsengard { .. } => write!(f, "pippin, warden of isengard"),
            Self::PirImaginativeRascal { .. } => write!(f, "pir, imaginative rascal"),
            Self::ProudMentor { .. } => write!(f, "proud mentor"),
            Self::RegnaTheRedeemer { .. } => write!(f, "regnathe redeemer"),
            Self::RhodaGeistAvenger { .. } => write!(f, "rhoda, geist avenger"),
            Self::RocksteadyMutantMarauder { .. } => write!(f, "rocksteady, mutant marauder"),
            Self::RoryWilliams { .. } => write!(f, "rory williams"),
            Self::RowanKenrith { .. } => write!(f, "rowan kenrith"),
            Self::SamLoyalAttendant { .. } => write!(f, "sam, loyal attendant"),
            Self::ShabraztheSkyshark { .. } => write!(f, "shabrazthe skyshark"),
            Self::SilvarDevoureroftheFree { .. } => write!(f, "silvar, devourer of the free"),
            Self::SoulbladeCorrupter { .. } => write!(f, "soulblade corrupter"),
            Self::SoulbladeRenewer { .. } => write!(f, "soulblade renewer"),
            Self::SylviaBrightspear { .. } => write!(f, "sylvia brightspear"),
            Self::TiminYouthfulGeist { .. } => write!(f, "timin, youthful geist"),
            Self::ToothyImaginaryFriend { .. } => write!(f, "toothy imaginary friend"),
            Self::TrynnChampionOfFreedom { .. } => write!(f, "trynn champion of freedom"),
            Self::UkkimaStalkingShadow { .. } => write!(f, "ukkima, stalking shadow"),
            Self::VirtustheVeiled { .. } => write!(f, "virtusthe veiled"),
            Self::WillKenrith { .. } => write!(f, "will kenrith"),
            Self::YannikScavengingSentinel { .. } => write!(f, "yannik, scavenging sentinel"),
            Self::ZndrspltEyeOfWisdom { .. } => write!(f, "zndrsplt, eyeof wisdom"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedPartner {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "alisaie leveilleur" => Some(NamedPartner::AlisaieLeveilleur {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "alphinaud leveilleur" => Some(NamedPartner::AlphinaudLeveilleur {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "amy pond" => Some(NamedPartner::AmyPond {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bebop, skull & crossbones" => Some(NamedPartner::BebopSkullAndCrossbones {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blaring captain" => Some(NamedPartner::BlaringCaptain {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blaring recruiter" => Some(NamedPartner::BlaringRecruiter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blue, loyal raptor" => Some(NamedPartner::BlueLoyalRaptor {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "brallin, skyshark rider" => Some(NamedPartner::BrallinSkysharkRider {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cazur, ruthless stalker" => Some(NamedPartner::CazurRuthlessStalker {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chakram retriever" => Some(NamedPartner::ChakramRetriever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chakram slinger" => Some(NamedPartner::ChakramSlinger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "evie frye" => Some(NamedPartner::EvieFrye {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "frodo, adventurous hobbit" => Some(NamedPartner::FrodoAdventurousHobbit {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "gorm the great" => Some(NamedPartner::GormtheGreat {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "haldan, avid arcanist" => Some(NamedPartner::HaldanAvidArcanist {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "impetuous protege" => Some(NamedPartner::ImpetuousProtege {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "jacob frye" => Some(NamedPartner::JacobFrye {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "jenny flint" => Some(NamedPartner::JennyFlint {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "kamber, the plunderer" => Some(NamedPartner::KamberThePlunderer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "khorvath brightflame" => Some(NamedPartner::KhorvathBrightflame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "krav, the unredeemed" => Some(NamedPartner::KravtheUnredeemed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "laurine, the diversion" => Some(NamedPartner::LaurineTheDiversion {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ley weaver" => Some(NamedPartner::LeyWeaver {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lore weaver" => Some(NamedPartner::LoreWeaver {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "madame vastra" => Some(NamedPartner::MadameVastra {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "merry, warden of isengard" => Some(NamedPartner::MerryWardenOfIsengard {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nikara, lair scavenger" => Some(NamedPartner::NikaraLairScavenger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "okaun, eye of chaos" => Some(NamedPartner::OkaunEyeOfChaos {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "owen grady, raptor trainer" => Some(NamedPartner::OwenGradyRaptorTrainer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pako, arcane retriever" => Some(NamedPartner::PakoArcaneRetriever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pippin, warden of isengard" => Some(NamedPartner::PippinWardenofIsengard {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pir, imaginative rascal" => Some(NamedPartner::PirImaginativeRascal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "proud mentor" => Some(NamedPartner::ProudMentor {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "regna, the redeemer" => Some(NamedPartner::RegnaTheRedeemer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rhoda, geist avenger" => Some(NamedPartner::RhodaGeistAvenger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rocksteady, mutant marauder" => Some(NamedPartner::RocksteadyMutantMarauder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rory williams" => Some(NamedPartner::RoryWilliams {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rowan kenrith" => Some(NamedPartner::RowanKenrith {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sam, loyal attendant" => Some(NamedPartner::SamLoyalAttendant {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "shabraz, the skyshark" => Some(NamedPartner::ShabraztheSkyshark {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "silvar, devourer of the free" => Some(NamedPartner::SilvarDevoureroftheFree {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "soulblade corrupter" => Some(NamedPartner::SoulbladeCorrupter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "soulblade renewer" => Some(NamedPartner::SoulbladeRenewer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sylvia brightspear" => Some(NamedPartner::SylviaBrightspear {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "timin, youthful geist" => Some(NamedPartner::TiminYouthfulGeist {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "toothy, imaginary friend" => Some(NamedPartner::ToothyImaginaryFriend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "trynn, champion of freedom" => Some(NamedPartner::TrynnChampionOfFreedom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ukkima, stalking shadow" => Some(NamedPartner::UkkimaStalkingShadow {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "virtus the veiled" => Some(NamedPartner::VirtustheVeiled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "will kenrith" => Some(NamedPartner::WillKenrith {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "yannik, scavenging sentinel" => Some(NamedPartner::YannikScavengingSentinel {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "zndrsplt, eye of wisdom" => Some(NamedPartner::ZndrspltEyeOfWisdom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
