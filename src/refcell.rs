//! source of inspiration :
//! https://rust-lang.github.io/async-book/02_execution/03_wakeups.html

use core::{cell::UnsafeCell, ptr::NonNull, task::Waker};

/// TODO: document
#[derive(Debug)]
pub struct RefCell<T: ?Sized> {
    //borrow: Cell<BorrowFlag>,
    value: UnsafeCell<T>,
}
impl<T> RefCell<T> {
    pub const fn new(value: T) -> RefCell<T> {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    ///TODO:
    pub async fn into_inner(self) -> T {
        self.value.into_inner()
    }

    //pub fn replace(&self, t: T) -> T {}
    //pub fn replace_with<F>(&self, f: F) -> T
    //pub fn swap(&self, other: &RefCell<T>)

    ///
    pub async fn borrow(&self) -> Ref<'_, T> {}

    /// Mutably borrows the wrapped value.
    /// await for the resource availability if needed.
    pub async fn borrow_mut(&self) -> RefMut<'_, T> {}
}

pub struct Ref<'b, T: ?Sized + 'b> {
    value: NonNull<T>,
    waker: Waker,
}

pub struct RefMut<'b, T: ?Sized + 'b> {}
