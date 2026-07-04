#[derive(idris_derive::Idris)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerAction {
    Add {
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
    Guess {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    GuessWrong {
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
    Remove {
        #[cfg(feature = "spanned_tree")]
        span: crate::ability_tree::span::TreeSpan,
    },
    Reselect {
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
impl PlayerAction {
    pub fn span(&self) -> crate::ability_tree::span::TreeSpan {
        match self {
            Self::Add { span } => *span,
            Self::Begin { span } => *span,
            Self::BeginTheGameWith { span } => *span,
            Self::Bid { span } => *span,
            Self::Change { span } => *span,
            Self::Choose { span } => *span,
            Self::ChooseAnyNumber { span } => *span,
            Self::CommitACrime { span } => *span,
            Self::CompletedADungeon { span } => *span,
            Self::Count { span } => *span,
            Self::Cycle { span } => *span,
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
            Self::Guess { span } => *span,
            Self::GuessWrong { span } => *span,
            Self::LookAt { span } => *span,
            Self::Ignore { span } => *span,
            Self::Lock { span } => *span,
            Self::Move { span } => *span,
            Self::Mulligan { span } => *span,
            Self::Note { span } => *span,
            Self::Pay { span } => *span,
            Self::Prevent { span } => *span,
            Self::Redistribute { span } => *span,
            Self::RepeatThisProcess { span } => *span,
            Self::Replace { span } => *span,
            Self::Reorder { span } => *span,
            Self::Remove { span } => *span,
            Self::Reselect { span } => *span,
            Self::Roll { span } => *span,
            Self::SecretlyChoose { span } => *span,
            Self::SecretlyVote { span } => *span,
            Self::Separate { span } => *span,
            Self::Shuffle { span } => *span,
            Self::Simultaneously { span } => *span,
            Self::Skip { span } => *span,
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

impl PlayerAction {
    pub fn try_from_span(span: &crate::lexer::Span) -> Option<Self> {
        match span.text {
            "add" | "adds" => Some(Self::Add {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "begin" | "begins" => Some(Self::Begin {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "begin the game with" => Some(Self::BeginTheGameWith {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "bid" => Some(Self::Bid {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "change" | "changes" => Some(Self::Change {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "choose" | "chooses" | "chose" | "chosen" | "choosing" => Some(Self::Choose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "choose any number" => Some(Self::ChooseAnyNumber {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "commit a crime" | "committed a crime" => Some(Self::CommitACrime {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "complete a dungeon" | "completed a dungeon" => Some(Self::CompletedADungeon {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "count" => Some(Self::Count {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "cycle" | "cycles" | "cycled" => Some(Self::Cycle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "distribute" => Some(Self::Distribute {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "draft" | "drafted" => Some(Self::Draft {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "draw" | "draws" | "drawn" | "drew" | "drawing" => Some(Self::Draw {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "encounter" => Some(Self::Encounter {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "expend" => Some(Self::Expend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "face a villanous choice" | "faces a villainous choice" => Some(Self::FaceAVillanousChoice {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "finish voting" => Some(Self::FinishVoting {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "flip" | "flips" => Some(Self::Flip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "fully unlock a room" => Some(Self::FullyUnlockARoom {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "give" => Some(Self::Give {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "guess" | "guesses" | "guessed" => Some(Self::Guess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "guess wrong" | "guessed wrong" => Some(Self::GuessWrong {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "lock" => Some(Self::Lock {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "ignore" => Some(Self::Ignore {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "look at" | "looks at" => Some(Self::LookAt {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "move" | "moves" => Some(Self::Move {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "mulligan" => Some(Self::Mulligan {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "note" | "noted" => Some(Self::Note {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "pay" | "pays" | "paying" | "paid" => Some(Self::Pay {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "prevent" | "prevented" => Some(Self::Prevent {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "redistribute" => Some(Self::Redistribute {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "repeat this process" | "repeat the following process" => Some(Self::RepeatThisProcess {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "replace" | "replacing" => Some(Self::Replace {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "reorder" => Some(Self::Reorder {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "remove" | "removing" | "removed" => Some(Self::Remove {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "reselect" => Some(Self::Reselect {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "roll" | "rolls" => Some(Self::Roll {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "secretly choose" | "secretly chooses" => Some(Self::SecretlyChoose {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "secretly vote" | "secretly votes" => Some(Self::SecretlyVote {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "separate" | "separates" => Some(Self::Separate {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "shuffle" | "shuffles" | "shuffled" => Some(Self::Shuffle {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "simultaneously" => Some(Self::Simultaneously {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "skip" | "skips" => Some(Self::Skip {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "spend" | "spends" | "spent" => Some(Self::Spend {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "start the bidding" => Some(Self::StartTheBidding {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "switch" => Some(Self::Switch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "take" | "takes" => Some(Self::Take {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "the ring tempts you" | "the ring has tempted you" => Some(Self::TheRingTemptsYou {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unattach" => Some(Self::Unattach {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "unlock" => Some(Self::Unlock {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "you won" => Some(Self::WonAClash {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            "you become the monarch" => Some(Self::YouBecomeTheMonarch {
                #[cfg(feature = "spanned_tree")]
                span: span.into(),
            }),
            _ => None,
        }
    }
}
