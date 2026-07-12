/// Fixme: doc
#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NamedPartner {
    AlisaieLeveilleur {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AlphinaudLeveilleur {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    AmyPond {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BebopSkullAndCrossbones {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BlaringCaptain {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BlaringRecruiter {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BlueLoyalRaptor {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BrallinSkysharkRider {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    CazurRuthlessStalker {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ChakramRetriever {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ChakramSlinger {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    EvieFrye {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FrodoAdventurousHobbit {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    GormtheGreat {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    HaldanAvidArcanist {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ImpetuousProtege {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    JacobFrye {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    JennyFlint {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    KamberThePlunderer {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    KhorvathBrightflame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    KravtheUnredeemed {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LaurineTheDiversion {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LeyWeaver {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LoreWeaver {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MadameVastra {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    MerryWardenOfIsengard {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    NikaraLairScavenger {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    OkaunEyeOfChaos {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    OwenGradyRaptorTrainer {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PakoArcaneRetriever {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PippinWardenofIsengard {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    PirImaginativeRascal {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ProudMentor {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RegnaTheRedeemer {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RhodaGeistAvenger {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RocksteadyMutantMarauder {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RoryWilliams {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RowanKenrith {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    SamLoyalAttendant {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ShabraztheSkyshark {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    SilvarDevoureroftheFree {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    SoulbladeCorrupter {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    SoulbladeRenewer {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    SylviaBrightspear {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TiminYouthfulGeist {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ToothyImaginaryFriend {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TrynnChampionOfFreedom {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    UkkimaStalkingShadow {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    VirtustheVeiled {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    WillKenrith {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    YannikScavengingSentinel {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ZndrspltEyeOfWisdom {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

impl Default for NamedPartner {
    fn default() -> Self {
        Self::ZndrspltEyeOfWisdom {
            #[cfg(feature = "spanned_tree")]
            span: Default::default(),
        }
    }
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for NamedPartner {
    fn span(&self) -> boseiju_span::Span {
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

impl<'src> TryFrom<&crate::LexerSpan<'src>> for NamedPartner {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "alisaie leveilleur" => Ok(NamedPartner::AlisaieLeveilleur {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "alphinaud leveilleur" => Ok(NamedPartner::AlphinaudLeveilleur {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "amy pond" => Ok(NamedPartner::AmyPond {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bebop, skull & crossbones" => Ok(NamedPartner::BebopSkullAndCrossbones {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blaring captain" => Ok(NamedPartner::BlaringCaptain {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blaring recruiter" => Ok(NamedPartner::BlaringRecruiter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blue, loyal raptor" => Ok(NamedPartner::BlueLoyalRaptor {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "brallin, skyshark rider" => Ok(NamedPartner::BrallinSkysharkRider {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cazur, ruthless stalker" => Ok(NamedPartner::CazurRuthlessStalker {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chakram retriever" => Ok(NamedPartner::ChakramRetriever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chakram slinger" => Ok(NamedPartner::ChakramSlinger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "evie frye" => Ok(NamedPartner::EvieFrye {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "frodo, adventurous hobbit" => Ok(NamedPartner::FrodoAdventurousHobbit {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "gorm the great" => Ok(NamedPartner::GormtheGreat {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "haldan, avid arcanist" => Ok(NamedPartner::HaldanAvidArcanist {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "impetuous protege" => Ok(NamedPartner::ImpetuousProtege {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "jacob frye" => Ok(NamedPartner::JacobFrye {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "jenny flint" => Ok(NamedPartner::JennyFlint {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "kamber, the plunderer" => Ok(NamedPartner::KamberThePlunderer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "khorvath brightflame" => Ok(NamedPartner::KhorvathBrightflame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "krav, the unredeemed" => Ok(NamedPartner::KravtheUnredeemed {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "laurine, the diversion" => Ok(NamedPartner::LaurineTheDiversion {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ley weaver" => Ok(NamedPartner::LeyWeaver {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lore weaver" => Ok(NamedPartner::LoreWeaver {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "madame vastra" => Ok(NamedPartner::MadameVastra {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "merry, warden of isengard" => Ok(NamedPartner::MerryWardenOfIsengard {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nikara, lair scavenger" => Ok(NamedPartner::NikaraLairScavenger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "okaun, eye of chaos" => Ok(NamedPartner::OkaunEyeOfChaos {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "owen grady, raptor trainer" => Ok(NamedPartner::OwenGradyRaptorTrainer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pako, arcane retriever" => Ok(NamedPartner::PakoArcaneRetriever {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pippin, warden of isengard" => Ok(NamedPartner::PippinWardenofIsengard {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pir, imaginative rascal" => Ok(NamedPartner::PirImaginativeRascal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "proud mentor" => Ok(NamedPartner::ProudMentor {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "regna, the redeemer" => Ok(NamedPartner::RegnaTheRedeemer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rhoda, geist avenger" => Ok(NamedPartner::RhodaGeistAvenger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rocksteady, mutant marauder" => Ok(NamedPartner::RocksteadyMutantMarauder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rory williams" => Ok(NamedPartner::RoryWilliams {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rowan kenrith" => Ok(NamedPartner::RowanKenrith {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sam, loyal attendant" => Ok(NamedPartner::SamLoyalAttendant {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "shabraz, the skyshark" => Ok(NamedPartner::ShabraztheSkyshark {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "silvar, devourer of the free" => Ok(NamedPartner::SilvarDevoureroftheFree {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "soulblade corrupter" => Ok(NamedPartner::SoulbladeCorrupter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "soulblade renewer" => Ok(NamedPartner::SoulbladeRenewer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sylvia brightspear" => Ok(NamedPartner::SylviaBrightspear {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "timin, youthful geist" => Ok(NamedPartner::TiminYouthfulGeist {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "toothy, imaginary friend" => Ok(NamedPartner::ToothyImaginaryFriend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "trynn, champion of freedom" => Ok(NamedPartner::TrynnChampionOfFreedom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ukkima, stalking shadow" => Ok(NamedPartner::UkkimaStalkingShadow {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "virtus the veiled" => Ok(NamedPartner::VirtustheVeiled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "will kenrith" => Ok(NamedPartner::WillKenrith {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "yannik, scavenging sentinel" => Ok(NamedPartner::YannikScavengingSentinel {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "zndrsplt, eye of wisdom" => Ok(NamedPartner::ZndrspltEyeOfWisdom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => Err(()),
        }
    }
}
