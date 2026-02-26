/// Special non terminal token for the clause that is seen quite a bit,
/// "in addition to paying its other cost".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InAdditionToPayingItsOtherCost;

impl InAdditionToPayingItsOtherCost {
    pub fn try_from_str(source: &str) -> Option<Self> {
        match source {
            "in addition to paying its other costs" => Some(Self),
            _ => None,
        }
    }
}

impl idris::Idris for InAdditionToPayingItsOtherCost {
    const COUNT: usize = 1;
    fn id(&self) -> usize {
        0
    }
    fn name_from_id(_: usize) -> &'static str {
        "InAdditionToPayingItsOtherCost"
    }
}
