pub fn dummy<T: DummyInit>() -> T {
    T::dummy_init()
}

/// Trait to create a meaningless instance of an object.
/// Since all parser nodes need to be constructible for id(), this allows to fill in their fields.
pub trait DummyInit {
    fn dummy_init() -> Self;
}

impl<T: DummyInit> DummyInit for Box<T> {
    fn dummy_init() -> Self {
        Box::new(dummy())
    }
}

impl<T, const N: usize> DummyInit for arrayvec::ArrayVec<T, N> {
    fn dummy_init() -> Self {
        Self::new_const()
    }
}

impl<T> DummyInit for Option<T> {
    fn dummy_init() -> Self {
        None
    }
}
