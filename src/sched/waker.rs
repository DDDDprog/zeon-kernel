/*

pub fn create_waker(work: Arc<Work>) -> Waker {
    let raw_waker = RawWaker::new(Arc::into_raw(work).cast(), &VTABLE);

    // SAFETY: We have correctly implemented the VTable functions.
    unsafe { Waker::from_raw(raw_waker) }
}
