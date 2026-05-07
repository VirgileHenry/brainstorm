pub trait Idris {
    const COUNT: usize;
    fn id(&self) -> usize;
    fn name_from_id(id: usize) -> &'static str;
}

impl<T: Idris> Idris for Option<T> {
    const COUNT: usize = { T::COUNT + 1 };

    fn id(&self) -> usize {
        match self {
            None => 0,
            Some(t) => 1 + t.id(),
        }
    }

    fn name_from_id(id: usize) -> &'static str {
        match id {
            0 => "None",
            other => T::name_from_id(other - 1),
        }
    }
}
