use crate::ability_tree::AbilityTreeNode;
use crate::ability_tree::MAX_CHILDREN_PER_NODE;
use crate::lexer::IntoToken;

/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedPartners {
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
    BebopSkull {
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
    KamberthePlunderer {
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
    LaurinetheDiversion {
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
    MerryWardenofIsengard {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NikaraLairScavenger {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    OkaunEyeofChaos {
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
    RegnatheRedeemer {
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
    TrynnChampionofFreedom {
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
    ZndrspltEyeofWisdom {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

impl AbilityTreeNode for NamedPartners {
    fn node_id(&self) -> usize {
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;
        crate::ability_tree::NodeKind::Terminal(TerminalNodeKind::NamedPartnersIdMarker).id()
    }

    fn children(&self) -> arrayvec::ArrayVec<&dyn AbilityTreeNode, MAX_CHILDREN_PER_NODE> {
        use crate::ability_tree::NodeKind;
        use crate::ability_tree::tree_node::TerminalNodeKind;
        use idris::Idris;

        let mut children = arrayvec::ArrayVec::new_const();
        let child_id = NodeKind::Terminal(TerminalNodeKind::NamedPartners(*self)).id();
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
            Self::BebopSkull { span } => *span,
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
            Self::KamberthePlunderer { span } => *span,
            Self::KhorvathBrightflame { span } => *span,
            Self::KravtheUnredeemed { span } => *span,
            Self::LaurinetheDiversion { span } => *span,
            Self::LeyWeaver { span } => *span,
            Self::LoreWeaver { span } => *span,
            Self::MadameVastra { span } => *span,
            Self::MerryWardenofIsengard { span } => *span,
            Self::NikaraLairScavenger { span } => *span,
            Self::OkaunEyeofChaos { span } => *span,
            Self::OwenGradyRaptorTrainer { span } => *span,
            Self::PakoArcaneRetriever { span } => *span,
            Self::PippinWardenofIsengard { span } => *span,
            Self::PirImaginativeRascal { span } => *span,
            Self::ProudMentor { span } => *span,
            Self::RegnatheRedeemer { span } => *span,
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
            Self::TrynnChampionofFreedom { span } => *span,
            Self::UkkimaStalkingShadow { span } => *span,
            Self::VirtustheVeiled { span } => *span,
            Self::WillKenrith { span } => *span,
            Self::YannikScavengingSentinel { span } => *span,
            Self::ZndrspltEyeofWisdom { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedPartners {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlisaieLeveilleur { .. } => write!(f, "alisaie leveilleur"),
            Self::AlphinaudLeveilleur { .. } => write!(f, "alphinaud leveilleur"),
            Self::AmyPond { .. } => write!(f, "amy pond"),
            Self::BebopSkull { .. } => write!(f, "bebop skull"),
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
            Self::KamberthePlunderer { .. } => write!(f, "kamberthe plunderer"),
            Self::KhorvathBrightflame { .. } => write!(f, "khorvath brightflame"),
            Self::KravtheUnredeemed { .. } => write!(f, "kravthe unredeemed"),
            Self::LaurinetheDiversion { .. } => write!(f, "laurinethe diversion"),
            Self::LeyWeaver { .. } => write!(f, "ley weaver"),
            Self::LoreWeaver { .. } => write!(f, "lore weaver"),
            Self::MadameVastra { .. } => write!(f, "madame vastra"),
            Self::MerryWardenofIsengard { .. } => write!(f, "merry, warden of isengard"),
            Self::NikaraLairScavenger { .. } => write!(f, "nikara, lair scavenger"),
            Self::OkaunEyeofChaos { .. } => write!(f, "okaun, eyeof chaos"),
            Self::OwenGradyRaptorTrainer { .. } => write!(f, "owen, grady raptor trainer"),
            Self::PakoArcaneRetriever { .. } => write!(f, "pako, arcane retriever"),
            Self::PippinWardenofIsengard { .. } => write!(f, "pippin, warden of isengard"),
            Self::PirImaginativeRascal { .. } => write!(f, "pir, imaginative rascal"),
            Self::ProudMentor { .. } => write!(f, "proud mentor"),
            Self::RegnatheRedeemer { .. } => write!(f, "regnathe redeemer"),
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
            Self::TrynnChampionofFreedom { .. } => write!(f, "trynn champion of freedom"),
            Self::UkkimaStalkingShadow { .. } => write!(f, "ukkima, stalking shadow"),
            Self::VirtustheVeiled { .. } => write!(f, "virtusthe veiled"),
            Self::WillKenrith { .. } => write!(f, "will kenrith"),
            Self::YannikScavengingSentinel { .. } => write!(f, "yannik, scavenging sentinel"),
            Self::ZndrspltEyeofWisdom { .. } => write!(f, "zndrsplt, eyeof wisdom"),
        }
    }
}

#[cfg(feature = "lexer")]
impl IntoToken for NamedPartners {
    fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "alisaie leveilleur" => Some(NamedPartners::AlisaieLeveilleur {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "alphinaud leveilleur" => Some(NamedPartners::AlphinaudLeveilleur {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "amy pond" => Some(NamedPartners::AmyPond {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bebop skull" => Some(NamedPartners::BebopSkull {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blaring captain" => Some(NamedPartners::BlaringCaptain {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blaring recruiter" => Some(NamedPartners::BlaringRecruiter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blue, loyal raptor" => Some(NamedPartners::BlueLoyalRaptor {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "brallin, skyshark rider" => Some(NamedPartners::BrallinSkysharkRider {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cazur, ruthless stalker" => Some(NamedPartners::CazurRuthlessStalker {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chakram retriever" => Some(NamedPartners::ChakramRetriever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chakram slinger" => Some(NamedPartners::ChakramSlinger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "evie frye" => Some(NamedPartners::EvieFrye {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "frodo, adventurous hobbit" => Some(NamedPartners::FrodoAdventurousHobbit {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "gormthe great" => Some(NamedPartners::GormtheGreat {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "haldan, avid arcanist" => Some(NamedPartners::HaldanAvidArcanist {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "impetuous protege" => Some(NamedPartners::ImpetuousProtege {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "jacob frye" => Some(NamedPartners::JacobFrye {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "jenny flint" => Some(NamedPartners::JennyFlint {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "kamberthe plunderer" => Some(NamedPartners::KamberthePlunderer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "khorvath brightflame" => Some(NamedPartners::KhorvathBrightflame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "kravthe unredeemed" => Some(NamedPartners::KravtheUnredeemed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "laurinethe diversion" => Some(NamedPartners::LaurinetheDiversion {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ley weaver" => Some(NamedPartners::LeyWeaver {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lore weaver" => Some(NamedPartners::LoreWeaver {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "madame vastra" => Some(NamedPartners::MadameVastra {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "merry, warden of isengard" => Some(NamedPartners::MerryWardenofIsengard {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nikara, lair scavenger" => Some(NamedPartners::NikaraLairScavenger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "okaun, eyeof chaos" => Some(NamedPartners::OkaunEyeofChaos {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "owen, grady raptor trainer" => Some(NamedPartners::OwenGradyRaptorTrainer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pako, arcane retriever" => Some(NamedPartners::PakoArcaneRetriever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pippin, warden of isengard" => Some(NamedPartners::PippinWardenofIsengard {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pir, imaginative rascal" => Some(NamedPartners::PirImaginativeRascal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "proud mentor" => Some(NamedPartners::ProudMentor {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "regnathe redeemer" => Some(NamedPartners::RegnatheRedeemer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rhoda, geist avenger" => Some(NamedPartners::RhodaGeistAvenger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rocksteady, mutant marauder" => Some(NamedPartners::RocksteadyMutantMarauder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rory williams" => Some(NamedPartners::RoryWilliams {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rowan kenrith" => Some(NamedPartners::RowanKenrith {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sam, loyal attendant" => Some(NamedPartners::SamLoyalAttendant {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "shabrazthe skyshark" => Some(NamedPartners::ShabraztheSkyshark {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "silvar, devourer of the free" => Some(NamedPartners::SilvarDevoureroftheFree {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "soulblade corrupter" => Some(NamedPartners::SoulbladeCorrupter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "soulblade renewer" => Some(NamedPartners::SoulbladeRenewer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sylvia brightspear" => Some(NamedPartners::SylviaBrightspear {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "timin, youthful geist" => Some(NamedPartners::TiminYouthfulGeist {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "toothy imaginary friend" => Some(NamedPartners::ToothyImaginaryFriend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "trynn champion of freedom" => Some(NamedPartners::TrynnChampionofFreedom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ukkima, stalking shadow" => Some(NamedPartners::UkkimaStalkingShadow {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "virtusthe veiled" => Some(NamedPartners::VirtustheVeiled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "will kenrith" => Some(NamedPartners::WillKenrith {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "yannik, scavenging sentinel" => Some(NamedPartners::YannikScavengingSentinel {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "zndrsplt, eyeof wisdom" => Some(NamedPartners::ZndrspltEyeofWisdom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
