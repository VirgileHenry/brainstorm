pub struct ManaCost(arrayvec::ArrayVec<mtg_data::Mana, 16>);

impl std::ops::Deref for ManaCost {
    type Target = [mtg_data::Mana];
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl std::ops::DerefMut for ManaCost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

impl std::str::FromStr for ManaCost {
    type Err = String; // Fixme!
    fn from_str(raw_mana_cost: &str) -> Result<Self, Self::Err> {
        let mut result = arrayvec::ArrayVec::new();

        /* Yeah, yeah, it's not that hard and may not need a regex. Whatever for now. */
        let mana_cost_regex = regex::Regex::new(r"(\{[^{}]+\})")
            .map_err(|e| format!("Failed to compile the mana cost iterator regex: {e}"))?;

        for capture in mana_cost_regex.captures_iter(raw_mana_cost) {
            let mana = mtg_data::Mana::from_str(capture.get_match().as_str())
                .map_err(|e| format!("Failed to parse captured mana cost: {e}"))?;
            result.push(mana);
        }

        Ok(ManaCost(result))
    }
}
