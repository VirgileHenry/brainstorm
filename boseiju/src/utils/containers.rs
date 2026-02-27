#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "ts_export", derive(ts_rs::TS))]
pub struct HeapArrayVec<T, const CAP: usize> {
    inner: Box<arrayvec::ArrayVec<T, CAP>>,
}

impl<T, const CAP: usize> HeapArrayVec<T, CAP> {
    pub fn new() -> Self {
        // let layout = std::alloc::Layout::new::<Box<arrayvec::ArrayVec<T, CAP>>>();
        // let allocation = unsafe { std::alloc::alloc(layout) };
        //
        // let mut result = unsafe { Box::from_raw(allocation as *mut arrayvec::ArrayVec<T, CAP>) };
        // unsafe { result.set_len(0) };

        /* Fixme: this seems to work for now ? */
        /* but it causes stack overflows if the array is too big and the compiler can't optimize it away */
        let result = Box::new(arrayvec::ArrayVec::new_const());

        Self { inner: result }
    }
}

impl<T, const CAP: usize> std::ops::Deref for HeapArrayVec<T, CAP> {
    type Target = arrayvec::ArrayVec<T, CAP>;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T, const CAP: usize> std::ops::DerefMut for HeapArrayVec<T, CAP> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl<T, const CAP: usize> crate::utils::DummyInit for HeapArrayVec<T, CAP> {
    fn dummy_init() -> Self {
        Self::new()
    }
}
