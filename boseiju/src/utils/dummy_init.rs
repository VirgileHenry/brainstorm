pub fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

/// Trait to create a meaningless instance of an object.
/// Since all parser nodes need to be constructible for id(), this allows to fill in their fields.
pub trait DummyInit {
    fn dummy_init() -> Self;
}
