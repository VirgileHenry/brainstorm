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
    AjaniInspiringLeader {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AlpineWatchdog {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AltanakTheThriceCalled {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ArachnusWeb {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ArgothSanctumOfNature {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    AshiokSculptorOfFears {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BasriDevotedPaladin {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BlackLotus {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BloodArtist {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BogbrewWitch {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BoulderbornDragon {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Braingeyser {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BrambleweftBehemoth {
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
    BrunaTheFadingLight {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BubblingCauldron {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChandraBoldPyromancer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChandraFlamesCatalyst {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChandraFlamesFury {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChandraPyrogenius {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    CommandTower {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    CrownOfEmpires {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Disenchant {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    DomriCitySmasher {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    DovinArchitectOfLaw {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    DragonstormGlobe {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    EightAndAHalfTails {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    EivorBattleReady {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    EmpyrialArchangel {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    EyeOfVecna {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    EzioBladeOfVengeance {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FangFearlessLCie {
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
    FlameBurst {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    GalactusDevourerOfWorlds {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    GarrukSavageHerald {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    GideonMartialParagon {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    GideonTheOathsworn {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    GodPharaohsGift {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Godsire {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    GroomsFinery {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HalvarGodOfBattle {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HammerOfNazahn {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HandOfVecna {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HanweirGarrison {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HeartPiercerBow {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HellkiteOverlord {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    HelmOfKaldra {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    IgneousCur {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    JaceArcaneStrategist {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    JaceIngeniousMindMage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    JiangYanggu {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    KeeperOfKookus {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LabyrinthOfSkophos {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LightningBolt {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LilianaDeathMage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LilianaDeathWielder {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MagnifyingGlass {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MidnightClock {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MidnightScavengers {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MineWorker {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MonkOfTheOpenHand {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    MuscleBurst {
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
    NicolBolasTheDeceiver {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NissaGenesisMage {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    NissaNaturesArtisan {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    OkoTheTrickster {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PeerThroughDepths {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PhyrexianDragonEngine {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PlantWorker {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    PrinceOfThralls {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RalCallerOfStorms {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ReachThroughMists {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Regrowth {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RowanFearlessSparkmage {
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
    ShivanDragon {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SilverSurferGalactussHerald {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SorinVampireLord {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SphinxSovereign {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SpittingDrake {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SunlitHoplite {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SwordOfKaldra {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Tardis {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TeferiTimebender {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TeferiTimelessVoyager {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Terror {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TezzeretMasterOfMetal {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheAnimus {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheMightstoneAndWeakstone {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheSpearOfLeonidas {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheUnderworldCookbook {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheUnspeakable {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ThinkingCap {
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
    VialOfDragonfire {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ViashivanDragon {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    VivienNaturesAvenger {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    VraskaRegalGorgon {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    VraskaSchemingGorgon {
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
            Self::AjaniInspiringLeader { span } => *span,
            Self::AlpineWatchdog { span } => *span,
            Self::AltanakTheThriceCalled { span } => *span,
            Self::ArachnusWeb { span } => *span,
            Self::ArgothSanctumOfNature { span } => *span,
            Self::AshiokSculptorOfFears { span } => *span,
            Self::BasriDevotedPaladin { span } => *span,
            Self::BlackLotus { span } => *span,
            Self::BloodArtist { span } => *span,
            Self::BogbrewWitch { span } => *span,
            Self::BoulderbornDragon { span } => *span,
            Self::Braingeyser { span } => *span,
            Self::BrambleweftBehemoth { span } => *span,
            Self::Breathstealer { span } => *span,
            Self::BridesGown { span } => *span,
            Self::BrunaTheFadingLight { span } => *span,
            Self::BubblingCauldron { span } => *span,
            Self::ChandraBoldPyromancer { span } => *span,
            Self::ChandraFlamesCatalyst { span } => *span,
            Self::ChandraFlamesFury { span } => *span,
            Self::ChandraPyrogenius { span } => *span,
            Self::CommandTower { span } => *span,
            Self::CrownOfEmpires { span } => *span,
            Self::Disenchant { span } => *span,
            Self::DomriCitySmasher { span } => *span,
            Self::DovinArchitectOfLaw { span } => *span,
            Self::DragonstormGlobe { span } => *span,
            Self::EightAndAHalfTails { span } => *span,
            Self::EivorBattleReady { span } => *span,
            Self::EmpyrialArchangel { span } => *span,
            Self::EyeOfVecna { span } => *span,
            Self::EzioBladeOfVengeance { span } => *span,
            Self::FangFearlessLCie { span } => *span,
            Self::FeralShadow { span } => *span,
            Self::FesteringNewt { span } => *span,
            Self::FlameBurst { span } => *span,
            Self::GalactusDevourerOfWorlds { span } => *span,
            Self::GarrukSavageHerald { span } => *span,
            Self::GideonMartialParagon { span } => *span,
            Self::GideonTheOathsworn { span } => *span,
            Self::GodPharaohsGift { span } => *span,
            Self::Godsire { span } => *span,
            Self::GroomsFinery { span } => *span,
            Self::HalvarGodOfBattle { span } => *span,
            Self::HammerOfNazahn { span } => *span,
            Self::HandOfVecna { span } => *span,
            Self::HanweirGarrison { span } => *span,
            Self::HeartPiercerBow { span } => *span,
            Self::HellkiteOverlord { span } => *span,
            Self::HelmOfKaldra { span } => *span,
            Self::IgneousCur { span } => *span,
            Self::JaceArcaneStrategist { span } => *span,
            Self::JaceIngeniousMindMage { span } => *span,
            Self::JiangYanggu { span } => *span,
            Self::KeeperOfKookus { span } => *span,
            Self::LabyrinthOfSkophos { span } => *span,
            Self::LightningBolt { span } => *span,
            Self::LilianaDeathMage { span } => *span,
            Self::LilianaDeathWielder { span } => *span,
            Self::MagnifyingGlass { span } => *span,
            Self::MidnightClock { span } => *span,
            Self::MidnightScavengers { span } => *span,
            Self::MineWorker { span } => *span,
            Self::MonkOfTheOpenHand { span } => *span,
            Self::MuscleBurst { span } => *span,
            Self::MuYanling { span } => *span,
            Self::MuYanlingCelestialWind { span } => *span,
            Self::NicolBolasTheDeceiver { span } => *span,
            Self::NissaGenesisMage { span } => *span,
            Self::NissaNaturesArtisan { span } => *span,
            Self::OkoTheTrickster { span } => *span,
            Self::PeerThroughDepths { span } => *span,
            Self::PhyrexianDragonEngine { span } => *span,
            Self::PlantWorker { span } => *span,
            Self::PrinceOfThralls { span } => *span,
            Self::RalCallerOfStorms { span } => *span,
            Self::ReachThroughMists { span } => *span,
            Self::Regrowth { span } => *span,
            Self::RowanFearlessSparkmage { span } => *span,
            Self::ScepterOfEmpires { span } => *span,
            Self::ShieldOfKaldra { span } => *span,
            Self::ShivanDragon { span } => *span,
            Self::SilverSurferGalactussHerald { span } => *span,
            Self::SorinVampireLord { span } => *span,
            Self::SphinxSovereign { span } => *span,
            Self::SpittingDrake { span } => *span,
            Self::SunlitHoplite { span } => *span,
            Self::SwordOfKaldra { span } => *span,
            Self::Tardis { span } => *span,
            Self::TeferiTimebender { span } => *span,
            Self::TeferiTimelessVoyager { span } => *span,
            Self::Terror { span } => *span,
            Self::TezzeretMasterOfMetal { span } => *span,
            Self::TheAnimus { span } => *span,
            Self::TheMightstoneAndWeakstone { span } => *span,
            Self::TheSpearOfLeonidas { span } => *span,
            Self::TheUnderworldCookbook { span } => *span,
            Self::TheUnspeakable { span } => *span,
            Self::ThinkingCap { span } => *span,
            Self::ThroneOfEmpires { span } => *span,
            Self::TowerWorker { span } => *span,
            Self::VialOfDragonfire { span } => *span,
            Self::ViashivanDragon { span } => *span,
            Self::VivienNaturesAvenger { span } => *span,
            Self::VraskaRegalGorgon { span } => *span,
            Self::VraskaSchemingGorgon { span } => *span,
            Self::Wastes { span } => *span,
        }
    }
}

impl std::fmt::Display for NamedCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AdvocateOfTheBeast { .. } => write!(f, "accumulated knowledge"),
            Self::AjaniInspiringLeader { .. } => write!(f, "accumulated knowledge"),
            Self::AlpineWatchdog { .. } => write!(f, "accumulated knowledge"),
            Self::AltanakTheThriceCalled { .. } => write!(f, "accumulated knowledge"),
            Self::ArachnusWeb { .. } => write!(f, "accumulated knowledge"),
            Self::ArgothSanctumOfNature { .. } => write!(f, "accumulated knowledge"),
            Self::AshiokSculptorOfFears { .. } => write!(f, "accumulated knowledge"),
            Self::BasriDevotedPaladin { .. } => write!(f, "accumulated knowledge"),
            Self::BlackLotus { .. } => write!(f, "accumulated knowledge"),
            Self::BloodArtist { .. } => write!(f, "accumulated knowledge"),
            Self::BogbrewWitch { .. } => write!(f, "accumulated knowledge"),
            Self::BoulderbornDragon { .. } => write!(f, "accumulated knowledge"),
            Self::Braingeyser { .. } => write!(f, "accumulated knowledge"),
            Self::BrambleweftBehemoth { .. } => write!(f, "accumulated knowledge"),
            Self::Breathstealer { .. } => write!(f, "accumulated knowledge"),
            Self::BridesGown { .. } => write!(f, "accumulated knowledge"),
            Self::BrunaTheFadingLight { .. } => write!(f, "accumulated knowledge"),
            Self::BubblingCauldron { .. } => write!(f, "accumulated knowledge"),
            Self::ChandraBoldPyromancer { .. } => write!(f, "accumulated knowledge"),
            Self::ChandraFlamesCatalyst { .. } => write!(f, "accumulated knowledge"),
            Self::ChandraFlamesFury { .. } => write!(f, "accumulated knowledge"),
            Self::ChandraPyrogenius { .. } => write!(f, "accumulated knowledge"),
            Self::CommandTower { .. } => write!(f, "accumulated knowledge"),
            Self::CrownOfEmpires { .. } => write!(f, "accumulated knowledge"),
            Self::Disenchant { .. } => write!(f, "accumulated knowledge"),
            Self::DomriCitySmasher { .. } => write!(f, "accumulated knowledge"),
            Self::DovinArchitectOfLaw { .. } => write!(f, "accumulated knowledge"),
            Self::DragonstormGlobe { .. } => write!(f, "accumulated knowledge"),
            Self::EightAndAHalfTails { .. } => write!(f, "accumulated knowledge"),
            Self::EivorBattleReady { .. } => write!(f, "accumulated knowledge"),
            Self::EmpyrialArchangel { .. } => write!(f, "accumulated knowledge"),
            Self::EyeOfVecna { .. } => write!(f, "accumulated knowledge"),
            Self::EzioBladeOfVengeance { .. } => write!(f, "accumulated knowledge"),
            Self::FangFearlessLCie { .. } => write!(f, "accumulated knowledge"),
            Self::FeralShadow { .. } => write!(f, "accumulated knowledge"),
            Self::FesteringNewt { .. } => write!(f, "accumulated knowledge"),
            Self::FlameBurst { .. } => write!(f, "accumulated knowledge"),
            Self::GalactusDevourerOfWorlds { .. } => write!(f, "accumulated knowledge"),
            Self::GarrukSavageHerald { .. } => write!(f, "accumulated knowledge"),
            Self::GideonMartialParagon { .. } => write!(f, "accumulated knowledge"),
            Self::GideonTheOathsworn { .. } => write!(f, "accumulated knowledge"),
            Self::GodPharaohsGift { .. } => write!(f, "accumulated knowledge"),
            Self::Godsire { .. } => write!(f, "accumulated knowledge"),
            Self::GroomsFinery { .. } => write!(f, "accumulated knowledge"),
            Self::HalvarGodOfBattle { .. } => write!(f, "accumulated knowledge"),
            Self::HammerOfNazahn { .. } => write!(f, "accumulated knowledge"),
            Self::HandOfVecna { .. } => write!(f, "accumulated knowledge"),
            Self::HanweirGarrison { .. } => write!(f, "accumulated knowledge"),
            Self::HeartPiercerBow { .. } => write!(f, "accumulated knowledge"),
            Self::HellkiteOverlord { .. } => write!(f, "accumulated knowledge"),
            Self::HelmOfKaldra { .. } => write!(f, "accumulated knowledge"),
            Self::IgneousCur { .. } => write!(f, "accumulated knowledge"),
            Self::JaceArcaneStrategist { .. } => write!(f, "accumulated knowledge"),
            Self::JaceIngeniousMindMage { .. } => write!(f, "accumulated knowledge"),
            Self::JiangYanggu { .. } => write!(f, "accumulated knowledge"),
            Self::KeeperOfKookus { .. } => write!(f, "accumulated knowledge"),
            Self::LabyrinthOfSkophos { .. } => write!(f, "accumulated knowledge"),
            Self::LightningBolt { .. } => write!(f, "accumulated knowledge"),
            Self::LilianaDeathMage { .. } => write!(f, "accumulated knowledge"),
            Self::LilianaDeathWielder { .. } => write!(f, "accumulated knowledge"),
            Self::MagnifyingGlass { .. } => write!(f, "accumulated knowledge"),
            Self::MidnightClock { .. } => write!(f, "accumulated knowledge"),
            Self::MidnightScavengers { .. } => write!(f, "accumulated knowledge"),
            Self::MineWorker { .. } => write!(f, "accumulated knowledge"),
            Self::MonkOfTheOpenHand { .. } => write!(f, "accumulated knowledge"),
            Self::MuscleBurst { .. } => write!(f, "accumulated knowledge"),
            Self::MuYanling { .. } => write!(f, "accumulated knowledge"),
            Self::MuYanlingCelestialWind { .. } => write!(f, "accumulated knowledge"),
            Self::NicolBolasTheDeceiver { .. } => write!(f, "accumulated knowledge"),
            Self::NissaGenesisMage { .. } => write!(f, "accumulated knowledge"),
            Self::NissaNaturesArtisan { .. } => write!(f, "accumulated knowledge"),
            Self::OkoTheTrickster { .. } => write!(f, "accumulated knowledge"),
            Self::PeerThroughDepths { .. } => write!(f, "accumulated knowledge"),
            Self::PhyrexianDragonEngine { .. } => write!(f, "accumulated knowledge"),
            Self::PlantWorker { .. } => write!(f, "accumulated knowledge"),
            Self::PrinceOfThralls { .. } => write!(f, "accumulated knowledge"),
            Self::RalCallerOfStorms { .. } => write!(f, "accumulated knowledge"),
            Self::ReachThroughMists { .. } => write!(f, "accumulated knowledge"),
            Self::Regrowth { .. } => write!(f, "accumulated knowledge"),
            Self::RowanFearlessSparkmage { .. } => write!(f, "accumulated knowledge"),
            Self::ScepterOfEmpires { .. } => write!(f, "accumulated knowledge"),
            Self::ShieldOfKaldra { .. } => write!(f, "accumulated knowledge"),
            Self::ShivanDragon { .. } => write!(f, "accumulated knowledge"),
            Self::SilverSurferGalactussHerald { .. } => write!(f, "accumulated knowledge"),
            Self::SorinVampireLord { .. } => write!(f, "accumulated knowledge"),
            Self::SphinxSovereign { .. } => write!(f, "accumulated knowledge"),
            Self::SpittingDrake { .. } => write!(f, "accumulated knowledge"),
            Self::SunlitHoplite { .. } => write!(f, "accumulated knowledge"),
            Self::SwordOfKaldra { .. } => write!(f, "accumulated knowledge"),
            Self::Tardis { .. } => write!(f, "accumulated knowledge"),
            Self::TeferiTimebender { .. } => write!(f, "accumulated knowledge"),
            Self::TeferiTimelessVoyager { .. } => write!(f, "accumulated knowledge"),
            Self::Terror { .. } => write!(f, "accumulated knowledge"),
            Self::TezzeretMasterOfMetal { .. } => write!(f, "accumulated knowledge"),
            Self::TheAnimus { .. } => write!(f, "accumulated knowledge"),
            Self::TheMightstoneAndWeakstone { .. } => write!(f, "accumulated knowledge"),
            Self::TheSpearOfLeonidas { .. } => write!(f, "accumulated knowledge"),
            Self::TheUnderworldCookbook { .. } => write!(f, "accumulated knowledge"),
            Self::TheUnspeakable { .. } => write!(f, "accumulated knowledge"),
            Self::ThinkingCap { .. } => write!(f, "accumulated knowledge"),
            Self::ThroneOfEmpires { .. } => write!(f, "accumulated knowledge"),
            Self::TowerWorker { .. } => write!(f, "accumulated knowledge"),
            Self::VialOfDragonfire { .. } => write!(f, "accumulated knowledge"),
            Self::ViashivanDragon { .. } => write!(f, "accumulated knowledge"),
            Self::VivienNaturesAvenger { .. } => write!(f, "accumulated knowledge"),
            Self::VraskaRegalGorgon { .. } => write!(f, "accumulated knowledge"),
            Self::VraskaSchemingGorgon { .. } => write!(f, "accumulated knowledge"),
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
            "ajani, inspiring leader" => Some(Self::AjaniInspiringLeader {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "alpine watchdog" | "~ watchdog" => Some(Self::AlpineWatchdog {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "altanak, the thrice-called" => Some(Self::AltanakTheThriceCalled {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "arachnus web" | "~ web" => Some(Self::ArachnusWeb {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "argoth, sanctum of nature" => Some(Self::ArgothSanctumOfNature {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ashiok, sculptor of fears" => Some(Self::AshiokSculptorOfFears {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "basri, devoted paladin" => Some(Self::BasriDevotedPaladin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "black lotus" => Some(Self::BlackLotus {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "blood artist" => Some(Self::BloodArtist {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bogbrew witch" => Some(Self::BogbrewWitch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "boulderborn dragon" => Some(Self::BoulderbornDragon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "brambleweft behemoth" => Some(Self::BrambleweftBehemoth {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "braingeyser" => Some(Self::Braingeyser {
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
            "bruna, the fading light" => Some(Self::BrunaTheFadingLight {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bubbling cauldron" => Some(Self::BubblingCauldron {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chandra, bold pyromancer" => Some(Self::ChandraBoldPyromancer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chandra, flame's catalyst" => Some(Self::ChandraFlamesCatalyst {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chandra, flame's fury" => Some(Self::ChandraFlamesFury {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "chandra, pyrogenius" => Some(Self::ChandraPyrogenius {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "command tower" => Some(Self::CommandTower {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "crown of empires" => Some(Self::CrownOfEmpires {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "disenchant" => Some(Self::Disenchant {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "domri, city smasher" => Some(Self::DomriCitySmasher {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "dovin, architect of law" => Some(Self::DovinArchitectOfLaw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "dragonstorm globe" | "~ globe" => Some(Self::DragonstormGlobe {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "eight-and-a-half-tails" => Some(Self::EightAndAHalfTails {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "eivor, battle-ready" => Some(Self::EivorBattleReady {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "empyrial archangel" => Some(Self::EmpyrialArchangel {
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
            "flame burst" => Some(Self::FlameBurst {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "galactus, devourer of worlds" => Some(Self::GalactusDevourerOfWorlds {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "garruk, savage herald" => Some(Self::GarrukSavageHerald {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "gideon, martial paragon" => Some(Self::GideonMartialParagon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "gideon, the oathsworn" => Some(Self::GideonTheOathsworn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "god-pharaoh's gift" => Some(Self::GodPharaohsGift {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "godsire" => Some(Self::Godsire {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "groom's finery" => Some(Self::GroomsFinery {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "halvar, god of battle" => Some(Self::HalvarGodOfBattle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "hammer of nazahn" | "hammer of ~" => Some(Self::HammerOfNazahn {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "eye of vecna" => Some(Self::EyeOfVecna {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ezio, blade of vengeance" => Some(Self::EzioBladeOfVengeance {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fang, fearless l'cie" => Some(Self::FangFearlessLCie {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "hand of vecna" => Some(Self::HandOfVecna {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "hanweir garrison" => Some(Self::HanweirGarrison {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "heart-piercer bow" => Some(Self::HeartPiercerBow {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "hellkite overlord" => Some(Self::HellkiteOverlord {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "helm of kaldra" => Some(Self::HelmOfKaldra {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "igneous cur" => Some(Self::IgneousCur {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "jace, arcane strategist" => Some(Self::JaceArcaneStrategist {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "jace, ingenious mind-mage" => Some(Self::JaceIngeniousMindMage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "jiang yanggu" => Some(Self::JiangYanggu {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "keeper of kookus" | "keeper of ~" => Some(Self::KeeperOfKookus {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "labyrinth of skophos" | "labyrinth of ~" => Some(Self::LabyrinthOfSkophos {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lightning bolt" => Some(Self::LightningBolt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "liliana, death mage" => Some(Self::LilianaDeathMage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "liliana, death wielder" => Some(Self::LilianaDeathWielder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "magnifying glass" => Some(Self::MagnifyingGlass {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "midnight clock" => Some(Self::MidnightClock {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "midnight scavengers" => Some(Self::MidnightScavengers {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mine worker" => Some(Self::MineWorker {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "monk of the open hand" => Some(Self::MonkOfTheOpenHand {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "muscle burst" => Some(Self::MuscleBurst {
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
            "nicol bolas, the deceiver" => Some(Self::NicolBolasTheDeceiver {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nissa, genesis mage" => Some(Self::NissaGenesisMage {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "nissa, nature's artisan" => Some(Self::NissaNaturesArtisan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "oko, the trickster" => Some(Self::OkoTheTrickster {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "peer through depths" => Some(Self::PeerThroughDepths {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "phyrexian dragon engine" => Some(Self::PhyrexianDragonEngine {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "plant worker" => Some(Self::PlantWorker {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "prince of thralls" => Some(Self::PrinceOfThralls {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ral, caller of storms" => Some(Self::RalCallerOfStorms {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "reach through mists" => Some(Self::ReachThroughMists {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "regrowth" => Some(Self::Regrowth {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "rowan, fearless sparkmage" => Some(Self::RowanFearlessSparkmage {
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
            "shivan dragon" => Some(Self::ShivanDragon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "silver surfer, galactus's herald" | "silver surfer, ~'s herald" => Some(Self::SilverSurferGalactussHerald {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sorin, vampire lord" => Some(Self::SorinVampireLord {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sphinx sovereign" => Some(Self::SphinxSovereign {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "spitting drake" => Some(Self::SpittingDrake {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sunlit hoplite" => Some(Self::SunlitHoplite {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "sword of kaldra" => Some(Self::SwordOfKaldra {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tardis" => Some(Self::Tardis {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "teferi, timebender" => Some(Self::TeferiTimebender {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "teferi, timeless voyager" => Some(Self::TeferiTimelessVoyager {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "terror" => Some(Self::Terror {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "tezzeret, master of metal" => Some(Self::TezzeretMasterOfMetal {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the animus" => Some(Self::TheAnimus {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the mightstone and weakstone" => Some(Self::TheMightstoneAndWeakstone {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the spear of leonidas" => Some(Self::TheSpearOfLeonidas {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the underworld cookbook" => Some(Self::TheUnderworldCookbook {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the unspeakable" => Some(Self::TheUnspeakable {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "thinking cap" => Some(Self::ThinkingCap {
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
            "vial of dragonfire" => Some(Self::VialOfDragonfire {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "viashivan dragon" => Some(Self::ViashivanDragon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "vivien, nature's avenger" => Some(Self::VivienNaturesAvenger {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "vraska, regal gorgon" => Some(Self::VraskaRegalGorgon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "vraska, scheming gorgon" => Some(Self::VraskaSchemingGorgon {
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
