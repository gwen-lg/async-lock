//! source of inspiration :
//! https://rust-lang.github.io/async-book/02_execution/03_wakeups.html

use core::{
    cell::{Cell, UnsafeCell},
    fmt,
    future::Future,
    ops::Deref,
    ptr::NonNull,
    task::Waker,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BorrowFlag {
    Available,
    Read { count: u32 },
    Write,
}
/// TODO: document
#[derive(Debug)]
pub struct RefCell<T: ?Sized> {
    //borrow: Cell<BorrowFlag>,
    raw: RawRefCell,
    value: UnsafeCell<T>,
}
impl<T> RefCell<T> {
    pub const fn new(value: T) -> RefCell<T> {
        Self {
            borrow: Cell::new(BorrowFlag::Available),
            value: UnsafeCell::new(value),
        }
    }

    ///TODO:
    // pub fn into_inner(self) -> T {
    //     self.value.into_inner()
    // }

    //pub fn replace(&self, t: T) -> T {}
    //pub fn replace_with<F>(&self, f: F) -> T
    //pub fn swap(&self, other: &RefCell<T>)

    ///
    pub async fn borrow(&self) -> Borrow<T> {
        Borrow::new(self.value.get()) // self.borrow,
    }

    // /// Mutably borrows the wrapped value.
    // /// await for the resource availability if needed.
    //pub async fn borrow_mut(&self) -> RefMut<'_, T> {}
}

pub struct Borrow<T: ?Sized> {
    value: NonNull<T>,
    //waker: Waker,
}

impl<T: ?Sized> Borrow<T> {
    fn new(value: *mut T) -> Self {
        let value = unsafe { NonNull::new_unchecked(value) };
        Self { value }
    }
}

impl<'b, T: ?Sized> Future for Borrow<T> {
    type Output = Ref<'b, T>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        // match self.state {

        // }
    }
}

impl<T: ?Sized> fmt::Debug for Borrow<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Borrow { .. }")
    }
}

pub struct Ref<'b, T: ?Sized + 'b> {}
impl<'b, T: ?Sized> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &'b Self::Target {
        unsafe { self.value.as_ref() }
    }
}
//pub struct RefMut<'b, T: ?Sized + 'b> {}
