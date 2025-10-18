#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Imperative {
    DealsDamage {
        dealer: crate::ability_tree::object::ObjectReference,
        amount: crate::ability_tree::terminals::Number,
        to: crate::ability_tree::object::ObjectReference,
    },
    Destroy {
        object: crate::ability_tree::object::ObjectReference,
    },
    Put {
        amount: crate::ability_tree::terminals::Number,
        of: crate::ability_tree::terminals::Counter,
        on: crate::ability_tree::object::ObjectReference,
    },
    Return {
        object: crate::ability_tree::object::ObjectReference,
        from: crate::ability_tree::zone::ZoneReference,
        to: crate::ability_tree::zone::ZoneReference,
    },
    Sacrifice {
        object: crate::ability_tree::object::ObjectReference,
    },
}

impl crate::ability_tree::AbilityTreeImpl for Imperative {
    fn display<W: std::io::Write>(
        &self,
        out: &mut crate::utils::TreeFormatter<'_, W>,
    ) -> std::io::Result<()> {
        use std::io::Write;
        match self {
            Imperative::DealsDamage { dealer, amount, to } => {
                write!(out, "Deals Damage:")?;
                out.push_inter_branch()?;
                write!(out, "Object:")?;
                out.push_final_branch()?;
                dealer.display(out)?;
                out.pop_branch();
                out.next_inter_branch()?;
                write!(out, "Amount:")?;
                out.push_final_branch()?;
                write!(out, "{amount}")?;
                out.pop_branch();
                out.next_final_branch()?;
                write!(out, "To:")?;
                out.push_final_branch()?;
                to.display(out)?;
                out.pop_branch();
                out.pop_branch();
            }
            Imperative::Destroy { object } => {
                write!(out, "Destroy:")?;
                out.push_final_branch()?;
                write!(out, "Object:")?;
                out.push_final_branch()?;
                object.display(out)?;
                out.pop_branch();
                out.pop_branch();
            }
            Imperative::Put { amount, of, on } => {
                write!(out, "Put:")?;
                out.push_inter_branch()?;
                write!(out, "Number:")?;
                out.push_final_branch()?;
                write!(out, "{amount}")?;
                out.pop_branch();
                out.next_inter_branch()?;
                write!(out, "Of:")?;
                out.push_final_branch()?;
                write!(out, "{of}")?;
                out.pop_branch();
                out.next_final_branch()?;
                write!(out, "On:")?;
                out.push_final_branch()?;
                on.display(out)?;
                out.pop_branch();
                out.pop_branch();
            }
            Imperative::Return { object, from, to } => {
                write!(out, "Return:")?;
                out.push_inter_branch()?;
                write!(out, "Object:")?;
                out.push_final_branch()?;
                object.display(out)?;
                out.pop_branch();
                out.next_inter_branch()?;
                write!(out, "From:")?;
                out.push_final_branch()?;
                from.display(out)?;
                out.pop_branch();
                out.next_final_branch()?;
                write!(out, "To:")?;
                out.push_final_branch()?;
                to.display(out)?;
                out.pop_branch();
                out.pop_branch();
            }
            Imperative::Sacrifice { object } => {
                writeln!(out, "Sacrifice:")?;
                out.push_final_branch()?;
                writeln!(out, "Object:")?;
                out.push_final_branch()?;
                object.display(out)?;
                out.pop_branch();
                out.pop_branch();
            }
        }
        Ok(())
    }
}
