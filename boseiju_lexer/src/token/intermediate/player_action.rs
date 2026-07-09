use crate::token::tensed::Tensed;

#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerAction {
    Accept {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Add {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Ante {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Begin {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    BeginTheGameWith {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Bid {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Change {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Choose {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ChooseAnyNumber {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Circle {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    CommitACrime {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    CompletedADungeon {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Count {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Cycle {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Decide {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Determine {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Distribute {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Draft {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Draw {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Encounter {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Expend {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FaceAVillanousChoice {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FinishVoting {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Flip {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    FullyUnlockARoom {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Give {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Group {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Guess {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    GuessCorrectly {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    GuessWrong {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Hide {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Ignore {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Lock {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    LookAt {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Move {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Mulligan {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Note {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Offer {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Pass {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Pay {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Prevent {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Redistribute {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Remove {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RepeatThisProcess {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Replace {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Reorder {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Reselect {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    RestartTheGame {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    ReverseTheTurnOrder {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Roll {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    SecretlyChoose {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    SecretlyVote {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Separate {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Shuffle {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Simultaneously {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Skip {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    SolveACase {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Spend {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    StartTheBidding {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Switch {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Take {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    TheRingTemptsYou {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Unattach {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    Unlock {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    WonAClash {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
    YouBecomeTheMonarch {
        #[cfg(feature = "spanned_tree")]
        span: boseiju_span::Span,
    },
}

#[cfg(feature = "spanned_tree")]
impl boseiju_span::Spanned for PlayerAction {
    fn span(&self) -> boseiju_span::Span {
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

impl<'src> TryFrom<&crate::LexerSpan<'src>> for TensedPlayerAction {
    type Error = ();
    fn try_from(span: &crate::LexerSpan) -> Result<Self, ()> {
        match span.text {
            "accept" => Ok(Tensed::base_form(PlayerAction::Accept {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "add" => Ok(Tensed::base_form(PlayerAction::Add {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "adds" => Ok(Tensed::third_person_singular_present(PlayerAction::Add {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "added" => Ok(Tensed::simple_past(PlayerAction::Add {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "antes" => Ok(Tensed::third_person_singular_present(PlayerAction::Ante {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "begin" => Ok(Tensed::base_form(PlayerAction::Begin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "begins" => Ok(Tensed::third_person_singular_present(PlayerAction::Begin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "began" => Ok(Tensed::simple_past(PlayerAction::Begin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "begin the game with" => Ok(Tensed::base_form(PlayerAction::BeginTheGameWith {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "bid" => Ok(Tensed::base_form(PlayerAction::Bid {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "change" => Ok(Tensed::base_form(PlayerAction::Change {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "changes" => Ok(Tensed::third_person_singular_present(PlayerAction::Change {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "changing" => Ok(Tensed::present_participle(PlayerAction::Change {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "choose" => Ok(Tensed::base_form(PlayerAction::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "chooses" => Ok(Tensed::third_person_singular_present(PlayerAction::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "chose" => Ok(Tensed::simple_past(PlayerAction::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "chosen" => Ok(Tensed::past_participle(PlayerAction::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "choosing" => Ok(Tensed::present_participle(PlayerAction::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "choose any number" => Ok(Tensed::base_form(PlayerAction::ChooseAnyNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "circle" => Ok(Tensed::base_form(PlayerAction::Circle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "commit a crime" => Ok(Tensed::base_form(PlayerAction::CommitACrime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "commits a crime" => Ok(Tensed::third_person_singular_present(PlayerAction::CommitACrime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "committed a crime" => Ok(Tensed::simple_past(PlayerAction::CommitACrime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "complete" => Ok(Tensed::base_form(PlayerAction::CompletedADungeon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "completed" => Ok(Tensed::simple_past(PlayerAction::CompletedADungeon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "count" => Ok(Tensed::base_form(PlayerAction::Count {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "cycle" => Ok(Tensed::base_form(PlayerAction::Cycle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "cycles" => Ok(Tensed::third_person_singular_present(PlayerAction::Cycle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "cycled" => Ok(Tensed::simple_past(PlayerAction::Cycle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "decide" => Ok(Tensed::base_form(PlayerAction::Decide {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "decides" => Ok(Tensed::third_person_singular_present(PlayerAction::Decide {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "determine" => Ok(Tensed::base_form(PlayerAction::Determine {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "distribute" => Ok(Tensed::base_form(PlayerAction::Distribute {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "draft" => Ok(Tensed::base_form(PlayerAction::Draft {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "drafts" => Ok(Tensed::third_person_singular_present(PlayerAction::Draft {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "drafted" => Ok(Tensed::simple_past(PlayerAction::Draft {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "drafting" => Ok(Tensed::present_participle(PlayerAction::Draft {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "draw" => Ok(Tensed::base_form(PlayerAction::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "draws" => Ok(Tensed::third_person_singular_present(PlayerAction::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "drawn" => Ok(Tensed::past_participle(PlayerAction::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "drew" => Ok(Tensed::simple_past(PlayerAction::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "drawing" => Ok(Tensed::present_participle(PlayerAction::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "encounter" => Ok(Tensed::base_form(PlayerAction::Encounter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "expend" => Ok(Tensed::base_form(PlayerAction::Expend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "face a villainous choice" | "face that choice" => Ok(Tensed::base_form(PlayerAction::FaceAVillanousChoice {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "faces a villainous choice" => Ok(Tensed::third_person_singular_present(PlayerAction::FaceAVillanousChoice {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "finish voting" => Ok(Tensed::base_form(PlayerAction::FinishVoting {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "flip" => Ok(Tensed::base_form(PlayerAction::Flip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "flips" => Ok(Tensed::third_person_singular_present(PlayerAction::Flip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "fully unlock a room" => Ok(Tensed::base_form(PlayerAction::FullyUnlockARoom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "give" => Ok(Tensed::base_form(PlayerAction::Give {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "group" => Ok(Tensed::base_form(PlayerAction::Group {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "guess" => Ok(Tensed::base_form(PlayerAction::Guess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "guesses" => Ok(Tensed::third_person_singular_present(PlayerAction::Guess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "guessed" => Ok(Tensed::simple_past(PlayerAction::Guess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "guessed correctly" => Ok(Tensed::simple_past(PlayerAction::GuessCorrectly {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "guess wrong" => Ok(Tensed::base_form(PlayerAction::GuessWrong {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "guessed wrong" => Ok(Tensed::simple_past(PlayerAction::GuessWrong {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "hide" => Ok(Tensed::base_form(PlayerAction::Hide {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "hides" => Ok(Tensed::third_person_singular_present(PlayerAction::Hide {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "lock" => Ok(Tensed::base_form(PlayerAction::Lock {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "ignore" => Ok(Tensed::base_form(PlayerAction::Ignore {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "look at" => Ok(Tensed::base_form(PlayerAction::LookAt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "looks at" => Ok(Tensed::third_person_singular_present(PlayerAction::LookAt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "move" => Ok(Tensed::base_form(PlayerAction::Move {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "moves" => Ok(Tensed::third_person_singular_present(PlayerAction::Move {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "moved" => Ok(Tensed::simple_past(PlayerAction::Move {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "mulligan" => Ok(Tensed::base_form(PlayerAction::Mulligan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "mulligans" => Ok(Tensed::third_person_singular_present(PlayerAction::Mulligan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "note" => Ok(Tensed::base_form(PlayerAction::Note {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "noted" => Ok(Tensed::simple_past(PlayerAction::Note {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "offer" => Ok(Tensed::base_form(PlayerAction::Offer {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "pass" => Ok(Tensed::base_form(PlayerAction::Pass {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "passes" => Ok(Tensed::third_person_singular_present(PlayerAction::Pass {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "passed" => Ok(Tensed::simple_past(PlayerAction::Pass {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "pay" => Ok(Tensed::base_form(PlayerAction::Pay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "pays" => Ok(Tensed::third_person_singular_present(PlayerAction::Pay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "paying" => Ok(Tensed::present_participle(PlayerAction::Pay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "paid" => Ok(Tensed::simple_past(PlayerAction::Pay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "prevent" => Ok(Tensed::base_form(PlayerAction::Prevent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "prevented" => Ok(Tensed::simple_past(PlayerAction::Prevent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "redistribute" => Ok(Tensed::base_form(PlayerAction::Redistribute {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "repeat this process" | "repeat the following process" => Ok(Tensed::base_form(PlayerAction::RepeatThisProcess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "repeats this process" => Ok(Tensed::third_person_singular_present(PlayerAction::RepeatThisProcess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "replace" => Ok(Tensed::base_form(PlayerAction::Replace {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "replacing" => Ok(Tensed::present_participle(PlayerAction::Replace {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "reorder" => Ok(Tensed::base_form(PlayerAction::Reorder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "remove" => Ok(Tensed::base_form(PlayerAction::Remove {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "removing" => Ok(Tensed::present_participle(PlayerAction::Remove {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "removed" => Ok(Tensed::simple_past(PlayerAction::Remove {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "reselect" => Ok(Tensed::base_form(PlayerAction::Reselect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "restart the game" => Ok(Tensed::base_form(PlayerAction::RestartTheGame {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "reverse the game's turn order" => Ok(Tensed::base_form(PlayerAction::ReverseTheTurnOrder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "roll" => Ok(Tensed::base_form(PlayerAction::Roll {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "rolls" => Ok(Tensed::third_person_singular_present(PlayerAction::Roll {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "rolled" => Ok(Tensed::simple_past(PlayerAction::Roll {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "secretly choose" => Ok(Tensed::base_form(PlayerAction::SecretlyChoose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "secretly chooses" => Ok(Tensed::third_person_singular_present(PlayerAction::SecretlyChoose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "secretly vote" => Ok(Tensed::base_form(PlayerAction::SecretlyVote {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "secretly votes" => Ok(Tensed::third_person_singular_present(PlayerAction::SecretlyVote {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "separate" => Ok(Tensed::base_form(PlayerAction::Separate {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "separates" => Ok(Tensed::third_person_singular_present(PlayerAction::Separate {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "shuffle" => Ok(Tensed::base_form(PlayerAction::Shuffle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "shuffles" => Ok(Tensed::third_person_singular_present(PlayerAction::Shuffle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "shuffled" => Ok(Tensed::simple_past(PlayerAction::Shuffle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "simultaneously" => Ok(Tensed::base_form(PlayerAction::Simultaneously {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "skip" => Ok(Tensed::base_form(PlayerAction::Skip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "skips" => Ok(Tensed::third_person_singular_present(PlayerAction::Skip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "solve a case" => Ok(Tensed::base_form(PlayerAction::SolveACase {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "spend" => Ok(Tensed::base_form(PlayerAction::Spend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "spends" => Ok(Tensed::third_person_singular_present(PlayerAction::Spend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "spent" => Ok(Tensed::simple_past(PlayerAction::Spend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "spending" => Ok(Tensed::present_participle(PlayerAction::Spend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "start the bidding" => Ok(Tensed::base_form(PlayerAction::StartTheBidding {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "switch" => Ok(Tensed::base_form(PlayerAction::Switch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "take" => Ok(Tensed::base_form(PlayerAction::Take {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "takes" => Ok(Tensed::third_person_singular_present(PlayerAction::Take {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "taking" => Ok(Tensed::present_participle(PlayerAction::Take {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "the ring tempts you" => Ok(Tensed::base_form(PlayerAction::TheRingTemptsYou {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "the ring has tempted you" => Ok(Tensed::simple_past(PlayerAction::TheRingTemptsYou {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "unattach" => Ok(Tensed::base_form(PlayerAction::Unattach {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "unlock" => Ok(Tensed::base_form(PlayerAction::Unlock {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "you won" => Ok(Tensed::simple_past(PlayerAction::WonAClash {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            "you become the monarch" => Ok(Tensed::base_form(PlayerAction::YouBecomeTheMonarch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            })),
            _ => Err(()),
        }
    }
}
