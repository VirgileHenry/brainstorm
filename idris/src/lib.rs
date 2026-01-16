pub trait Idris<Repr> {
    // type Id;
    const COUNT: Repr;
    fn id(&self) -> Repr;
}
