use crate::lexer::tokens::tensed::Tensed;

#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerAction {
    Accept {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Add {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Ante {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Begin {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    BeginTheGameWith {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Bid {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Change {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Choose {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ChooseAnyNumber {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Circle {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    CommitACrime {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    CompletedADungeon {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Count {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Cycle {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Decide {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Determine {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Distribute {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Draft {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Draw {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Encounter {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Expend {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FaceAVillanousChoice {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FinishVoting {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Flip {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    FullyUnlockARoom {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Give {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Group {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Guess {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    GuessCorrectly {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    GuessWrong {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Hide {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Ignore {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Lock {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    LookAt {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Move {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Mulligan {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Note {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Offer {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Pass {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Pay {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Prevent {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Redistribute {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Remove {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RepeatThisProcess {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Replace {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Reorder {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Reselect {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    RestartTheGame {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    ReverseTheTurnOrder {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Roll {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SecretlyChoose {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SecretlyVote {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Separate {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Shuffle {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Simultaneously {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Skip {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    SolveACase {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Spend {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    StartTheBidding {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Switch {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Take {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    TheRingTemptsYou {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Unattach {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Unlock {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    WonAClash {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    YouBecomeTheMonarch {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
}

#[cfg(feature = "spanned_tree")]
impl crate::ability_tree::span::Spanned for PlayerAction {
    fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Accept { span } => *span,
            Self::Add { span } => *span,
            Self::Ante { span } => *span,
            Self::Begin { span } => *span,
            Self::BeginTheGameWith { span } => *span,
            Self::Bid { span } => *span,
            Self::Change { span } => *span,
            Self::Choose { span } => *span,
            Self::ChooseAnyNumber { span } => *span,
            Self::Circle { span } => *span,
            Self::CommitACrime { span } => *span,
            Self::CompletedADungeon { span } => *span,
            Self::Count { span } => *span,
            Self::Cycle { span } => *span,
            Self::Decide { span } => *span,
            Self::Determine { span } => *span,
            Self::Distribute { span } => *span,
            Self::Draft { span } => *span,
            Self::Draw { span } => *span,
            Self::Encounter { span } => *span,
            Self::Expend { span } => *span,
            Self::FaceAVillanousChoice { span } => *span,
            Self::FinishVoting { span } => *span,
            Self::Flip { span } => *span,
            Self::FullyUnlockARoom { span } => *span,
            Self::Give { span } => *span,
            Self::Group { span } => *span,
            Self::Guess { span } => *span,
            Self::GuessCorrectly { span } => *span,
            Self::GuessWrong { span } => *span,
            Self::Hide { span } => *span,
            Self::LookAt { span } => *span,
            Self::Ignore { span } => *span,
            Self::Lock { span } => *span,
            Self::Move { span } => *span,
            Self::Mulligan { span } => *span,
            Self::Note { span } => *span,
            Self::Offer { span } => *span,
            Self::Pass { span } => *span,
            Self::Pay { span } => *span,
            Self::Prevent { span } => *span,
            Self::Redistribute { span } => *span,
            Self::RepeatThisProcess { span } => *span,
            Self::Replace { span } => *span,
            Self::Reorder { span } => *span,
            Self::Remove { span } => *span,
            Self::Reselect { span } => *span,
            Self::RestartTheGame { span } => *span,
            Self::ReverseTheTurnOrder { span } => *span,
            Self::Roll { span } => *span,
            Self::SecretlyChoose { span } => *span,
            Self::SecretlyVote { span } => *span,
            Self::Separate { span } => *span,
            Self::Shuffle { span } => *span,
            Self::Simultaneously { span } => *span,
            Self::Skip { span } => *span,
            Self::SolveACase { span } => *span,
            Self::Spend { span } => *span,
            Self::StartTheBidding { span } => *span,
            Self::Switch { span } => *span,
            Self::Take { span } => *span,
            Self::TheRingTemptsYou { span } => *span,
            Self::Unattach { span } => *span,
            Self::Unlock { span } => *span,
            Self::WonAClash { span } => *span,
            Self::YouBecomeTheMonarch { span } => *span,
        }
    }
}

pub type TensedPlayerAction = Tensed<PlayerAction>;

impl TensedPlayerAction {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "accept" => Some(Tensed::base_form(PlayerAction::Accept {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "add" => Some(Tensed::base_form(PlayerAction::Add {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "adds" => Some(Tensed::third_person_singular_present(PlayerAction::Add {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "added" => Some(Tensed::simple_past(PlayerAction::Add {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "antes" => Some(Tensed::third_person_singular_present(PlayerAction::Ante {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "begin" => Some(Tensed::base_form(PlayerAction::Begin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "begins" => Some(Tensed::third_person_singular_present(PlayerAction::Begin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "began" => Some(Tensed::simple_past(PlayerAction::Begin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "begin the game with" => Some(Tensed::base_form(PlayerAction::BeginTheGameWith {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "bid" => Some(Tensed::base_form(PlayerAction::Bid {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "change" => Some(Tensed::base_form(PlayerAction::Change {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "changes" => Some(Tensed::third_person_singular_present(PlayerAction::Change {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "changing" => Some(Tensed::present_participle(PlayerAction::Change {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "choose" => Some(Tensed::base_form(PlayerAction::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "chooses" => Some(Tensed::third_person_singular_present(PlayerAction::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "chose" => Some(Tensed::simple_past(PlayerAction::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "chosen" => Some(Tensed::past_participle(PlayerAction::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "choosing" => Some(Tensed::present_participle(PlayerAction::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "choose any number" => Some(Tensed::base_form(PlayerAction::ChooseAnyNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "circle" => Some(Tensed::base_form(PlayerAction::Circle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "commit a crime" => Some(Tensed::base_form(PlayerAction::CommitACrime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "commits a crime" => Some(Tensed::third_person_singular_present(PlayerAction::CommitACrime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "committed a crime" => Some(Tensed::simple_past(PlayerAction::CommitACrime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "complete" => Some(Tensed::base_form(PlayerAction::CompletedADungeon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "completed" => Some(Tensed::simple_past(PlayerAction::CompletedADungeon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "count" => Some(Tensed::base_form(PlayerAction::Count {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "cycle" => Some(Tensed::base_form(PlayerAction::Cycle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "cycles" => Some(Tensed::third_person_singular_present(PlayerAction::Cycle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "cycled" => Some(Tensed::simple_past(PlayerAction::Cycle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "decide" => Some(Tensed::base_form(PlayerAction::Decide {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "decides" => Some(Tensed::third_person_singular_present(PlayerAction::Decide {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "determine" => Some(Tensed::base_form(PlayerAction::Determine {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "distribute" => Some(Tensed::base_form(PlayerAction::Distribute {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "draft" => Some(Tensed::base_form(PlayerAction::Draft {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "drafts" => Some(Tensed::third_person_singular_present(PlayerAction::Draft {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "drafted" => Some(Tensed::simple_past(PlayerAction::Draft {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "drafting" => Some(Tensed::present_participle(PlayerAction::Draft {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "draw" => Some(Tensed::base_form(PlayerAction::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "draws" => Some(Tensed::third_person_singular_present(PlayerAction::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "drawn" => Some(Tensed::past_participle(PlayerAction::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "drew" => Some(Tensed::simple_past(PlayerAction::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "drawing" => Some(Tensed::present_participle(PlayerAction::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "encounter" => Some(Tensed::base_form(PlayerAction::Encounter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "expend" => Some(Tensed::base_form(PlayerAction::Expend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "face a villainous choice" | "face that choice" => Some(Tensed::base_form(PlayerAction::FaceAVillanousChoice {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "faces a villainous choice" => Some(Tensed::third_person_singular_present(PlayerAction::FaceAVillanousChoice {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "finish voting" => Some(Tensed::base_form(PlayerAction::FinishVoting {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "flip" => Some(Tensed::base_form(PlayerAction::Flip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "flips" => Some(Tensed::third_person_singular_present(PlayerAction::Flip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "fully unlock a room" => Some(Tensed::base_form(PlayerAction::FullyUnlockARoom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "give" => Some(Tensed::base_form(PlayerAction::Give {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "group" => Some(Tensed::base_form(PlayerAction::Group {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "guess" => Some(Tensed::base_form(PlayerAction::Guess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "guesses" => Some(Tensed::third_person_singular_present(PlayerAction::Guess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "guessed" => Some(Tensed::simple_past(PlayerAction::Guess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "guessed correctly" => Some(Tensed::simple_past(PlayerAction::GuessCorrectly {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "guess wrong" => Some(Tensed::base_form(PlayerAction::GuessWrong {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "guessed wrong" => Some(Tensed::simple_past(PlayerAction::GuessWrong {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "hide" => Some(Tensed::base_form(PlayerAction::Hide {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "hides" => Some(Tensed::third_person_singular_present(PlayerAction::Hide {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "lock" => Some(Tensed::base_form(PlayerAction::Lock {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "ignore" => Some(Tensed::base_form(PlayerAction::Ignore {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "look at" => Some(Tensed::base_form(PlayerAction::LookAt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "looks at" => Some(Tensed::third_person_singular_present(PlayerAction::LookAt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "move" => Some(Tensed::base_form(PlayerAction::Move {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "moves" => Some(Tensed::third_person_singular_present(PlayerAction::Move {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "moved" => Some(Tensed::simple_past(PlayerAction::Move {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "mulligan" => Some(Tensed::base_form(PlayerAction::Mulligan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "mulligans" => Some(Tensed::third_person_singular_present(PlayerAction::Mulligan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "note" => Some(Tensed::base_form(PlayerAction::Note {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "noted" => Some(Tensed::simple_past(PlayerAction::Note {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "offer" => Some(Tensed::base_form(PlayerAction::Offer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "pass" => Some(Tensed::base_form(PlayerAction::Pass {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "passes" => Some(Tensed::third_person_singular_present(PlayerAction::Pass {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "passed" => Some(Tensed::simple_past(PlayerAction::Pass {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "pay" => Some(Tensed::base_form(PlayerAction::Pay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "pays" => Some(Tensed::third_person_singular_present(PlayerAction::Pay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "paying" => Some(Tensed::present_participle(PlayerAction::Pay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "paid" => Some(Tensed::simple_past(PlayerAction::Pay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "prevent" => Some(Tensed::base_form(PlayerAction::Prevent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "prevented" => Some(Tensed::simple_past(PlayerAction::Prevent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "redistribute" => Some(Tensed::base_form(PlayerAction::Redistribute {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "repeat this process" | "repeat the following process" => Some(Tensed::base_form(PlayerAction::RepeatThisProcess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "repeats this process" => Some(Tensed::third_person_singular_present(PlayerAction::RepeatThisProcess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "replace" => Some(Tensed::base_form(PlayerAction::Replace {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "replacing" => Some(Tensed::present_participle(PlayerAction::Replace {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "reorder" => Some(Tensed::base_form(PlayerAction::Reorder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "remove" => Some(Tensed::base_form(PlayerAction::Remove {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "removing" => Some(Tensed::present_participle(PlayerAction::Remove {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "removed" => Some(Tensed::simple_past(PlayerAction::Remove {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "reselect" => Some(Tensed::base_form(PlayerAction::Reselect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "restart the game" => Some(Tensed::base_form(PlayerAction::RestartTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "reverse the game's turn order" => Some(Tensed::base_form(PlayerAction::ReverseTheTurnOrder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "roll" => Some(Tensed::base_form(PlayerAction::Roll {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "rolls" => Some(Tensed::third_person_singular_present(PlayerAction::Roll {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "rolled" => Some(Tensed::simple_past(PlayerAction::Roll {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "secretly choose" => Some(Tensed::base_form(PlayerAction::SecretlyChoose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "secretly chooses" => Some(Tensed::third_person_singular_present(PlayerAction::SecretlyChoose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "secretly vote" => Some(Tensed::base_form(PlayerAction::SecretlyVote {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "secretly votes" => Some(Tensed::third_person_singular_present(PlayerAction::SecretlyVote {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "separate" => Some(Tensed::base_form(PlayerAction::Separate {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "separates" => Some(Tensed::third_person_singular_present(PlayerAction::Separate {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "shuffle" => Some(Tensed::base_form(PlayerAction::Shuffle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "shuffles" => Some(Tensed::third_person_singular_present(PlayerAction::Shuffle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "shuffled" => Some(Tensed::simple_past(PlayerAction::Shuffle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "simultaneously" => Some(Tensed::base_form(PlayerAction::Simultaneously {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "skip" => Some(Tensed::base_form(PlayerAction::Skip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "skips" => Some(Tensed::third_person_singular_present(PlayerAction::Skip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "solve a case" => Some(Tensed::base_form(PlayerAction::SolveACase {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "spend" => Some(Tensed::base_form(PlayerAction::Spend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "spends" => Some(Tensed::third_person_singular_present(PlayerAction::Spend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "spent" => Some(Tensed::simple_past(PlayerAction::Spend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "spending" => Some(Tensed::present_participle(PlayerAction::Spend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "start the bidding" => Some(Tensed::base_form(PlayerAction::StartTheBidding {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "switch" => Some(Tensed::base_form(PlayerAction::Switch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "take" => Some(Tensed::base_form(PlayerAction::Take {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "takes" => Some(Tensed::third_person_singular_present(PlayerAction::Take {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "taking" => Some(Tensed::present_participle(PlayerAction::Take {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "the ring tempts you" => Some(Tensed::base_form(PlayerAction::TheRingTemptsYou {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "the ring has tempted you" => Some(Tensed::simple_past(PlayerAction::TheRingTemptsYou {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "unattach" => Some(Tensed::base_form(PlayerAction::Unattach {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "unlock" => Some(Tensed::base_form(PlayerAction::Unlock {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "you won" => Some(Tensed::simple_past(PlayerAction::WonAClash {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "you become the monarch" => Some(Tensed::base_form(PlayerAction::YouBecomeTheMonarch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            _ => None,
        }
    }
}
