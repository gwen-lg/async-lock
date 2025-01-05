use core::sync::atomic::{AtomicUsize, Ordering};

use event_listener::{Event, EventListener};

const BORROW_MUT_BIT: usize = 1;
const ONE_BORROW: usize = 2;

pub(super) struct RawRefCell {
    /// Event triggered when the last borrow is dropped.
    no_borrows: Event,

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
    pub(super) const fn new() -> Self {
        Self {
            no_borrows: Event::new(),
            no_borrow_mut: Event::new(),
            state: AtomicUsize::new(0),
        }
    }

    pub(super) fn borrow(&self) -> RawBorrow<'_> {
        RawBorrow {
            lock: self,
            state: self.state.load(Ordering::Acquire), //TODO: no need atomic as single threaded ?
            listener: None,
        }
    }

    pub(super) fn try_borrow(&self) -> bool {
        let mut state = self.state.load(Ordering::Acquire);
        loop {
            // If there's a mutable borrow holding the lock or attempting to acquire it, we cannot acquire
            // a read lock here.
            if state & BORROW_MUT_BIT != 0 {
                return false;
            }

            // Make sure the number of borrows doesn't overflow.
            if state > isize::MAX as usize {
                crate::abort();
            }

            // Increment the number of readers.
            match self.state.compare_exchange(
                state,
                state + ONE_BORROW,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(s) => state = s,
            }
        }
    }

    pub(super) fn borrow_unlock(&self) {
        // Decrement the number of borrows.
        if self.state.fetch_sub(ONE_BORROW, Ordering::SeqCst) & !BORROW_MUT_BIT == ONE_BORROW {
            // If this was the last reader, trigger the "no borrows" event.
            self.no_borrows.notify(1);
        }
    }
}

pub(super) struct RawBorrow<'a> {
    // The lock that is being acquired.
    pub(super) lock: &'a RawRefCell,

    // The listener for the "no writers" event.
    listener: Option<EventListener>,

    // The last-observed state of the lock.
    state: usize,
    // // Making this type `!Unpin` enables future optimizations.
    // #[pin]
    // _pin: PhantomPinned
}

impl RawBorrow<'_> {
    pub fn try_borrow(&mut self) -> bool {
        //let this = self.project();
        if self.state & BORROW_MUT_BIT == 0 {
            // Make sure the number of readers doesn't overflow.
            if self.state > isize::MAX as usize {
                crate::abort();
            } else {
                match self.lock.state.compare_exchange(
                    self.state,
                    self.state + ONE_BORROW,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ) {
                    Ok(_) => return true,
                    Err(s) => self.state = s,
                }
            }
        } else {
            if self.listener.is_none() {
                self.listener = Some(self.lock.no_borrows.listen())
            } else {
                todo!()
            }
            self.state = self.lock.state.load(Ordering::Acquire);
        }
        false
    }
}
