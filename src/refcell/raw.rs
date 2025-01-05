use core::sync::atomic::AtomicUsize;

use event_listener::Event;

pub(super) struct RawRefCell {
    /// Event triggered when the last borrow is dropped.
    no_borrow: Event,

    /// Event triggered when the borrow_mut is dropped.
    no_borrow_mut: Event,

    /// Current state of the lock.
    ///
    /// The least significant bit (`WRITER_BIT`) is set to 1 when a writer is holding the lock or
    /// trying to acquire it.
    ///
    /// The upper bits contain the number of currently active readers. Each active reader
    /// increments the state by `ONE_READER`.
    state: AtomicUsize,
}

impl RawRefCell {
    pub(super) fn new() -> Self {
        todo!()
    }

    pub(super) fn try_borrow(&self) -> bool {
        todo!()
    }

    pub(super) fn borrow_unlock(&self) {
        todo!()
    }
}

pub(super) struct RawBorrow<'a> {
    // The lock that is being acquired.
    pub(super) lock: &'a RawRefCell,

    // The last-observed state of the lock.
    state: usize,
    // // Making this type `!Unpin` enables future optimizations.
    // #[pin]
    // _pin: PhantomPinned
}

impl RawBorrow<'_> {
    pub fn try_borrow(&self) -> bool {
        todo!()
    }
}
