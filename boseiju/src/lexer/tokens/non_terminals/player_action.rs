#[derive(idris_derive::Idris)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlayerAction {
    Add,
    Attach,
    Attack,
    Change,
    Choose,
    Distribute,
    Draw,
    Gain, /* Fixme: out of place, and need different tense forms */
    LookAt,
    Lose,
    Pay,
    Prevent,
    Put,
    Return,
    Remove,
    Search,
    Shuffle,
    Spend,
}

impl PlayerAction {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "add" | "adds" => Some(Self::Add),
            "attach" | "attaches" => Some(Self::Attach),
            "attack" | "attacks" | "attacked" => Some(Self::Attack),
            "change" | "changes" => Some(Self::Change),
            "choose" | "chooses" | "choice" => Some(Self::Choose),
            "distribute" => Some(Self::Distribute),
            "draw" | "draws" => Some(Self::Draw),
            "gain" | "gains" | "gained" => Some(Self::Gain),
            "look at" | "looks at" => Some(Self::LookAt),
            "lose" | "loses" => Some(Self::LookAt),
            "pay" | "pays" | "paying" => Some(Self::Pay),
            "prevent" | "prevented" => Some(Self::Prevent),
            "return" | "returns" => Some(Self::Return),
            "remove" | "removing" => Some(Self::Remove),
            "search" | "searchs" => Some(Self::Search),
            "shuffle" | "shuffles" => Some(Self::Shuffle),
            "spend" | "spends" => Some(Self::Spend),
            _ => None,
        }
    }
}
