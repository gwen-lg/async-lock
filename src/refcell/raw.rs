pub(super) struct RawRefCell {
    /// Event triggered when the last borrow is dropped.
    no_borrow: Event,

    /// Event triggered when the borrow_mut is dropped.
    no_borrow_mut: Event,

    /// Current state of the Cell.
    ///
    /// The least significant bit (`WRITER_BIT`) is set to 1 when a writer is holding the lock or
    /// trying to acquire it.
    ///
    /// The upper bits contain the number of currently active readers. Each active reader
    /// increments the state by `ONE_READER`.
    state: AtomicUsize,
}
