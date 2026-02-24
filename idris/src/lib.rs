pub trait Idris {
    const COUNT: usize;
    fn id(&self) -> usize;
    fn name_from_id(id: usize) -> &'static str;
}
