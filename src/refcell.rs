//! source of inspiration :
//! https://rust-lang.github.io/async-book/02_execution/03_wakeups.html

mod raw;

use core::{
    cell::UnsafeCell,
    fmt,
    future::Future,
    ops::Deref,
    ptr::NonNull,
    task::{Poll, Waker},
};
use raw::{RawBorrow, RawRefCell};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BorrowFlag {
    Available,
    Read { count: u32 },
    Write,
}
/// TODO: document
pub struct RefCell<T: ?Sized> {
    //borrow: Cell<BorrowFlag>,
    raw: RawRefCell,
    value: UnsafeCell<T>,
}
impl<T> RefCell<T> {
    pub fn new(value: T) -> RefCell<T> {
        Self {
            raw: RawRefCell::new(),
            //borrow: Cell::new(BorrowFlag::Available),
            value: UnsafeCell::new(value),
        }
    }
}

impl<T: ?Sized> RefCell<T> {
    ///TODO:
    // pub fn into_inner(self) -> T {
    //     self.value.into_inner()
    // }

    //pub fn replace(&self, t: T) -> T {}
    //pub fn replace_with<F>(&self, f: F) -> T
    //pub fn swap(&self, other: &RefCell<T>)

    ///
    pub fn borrow(&self) -> Borrow<T> {
        Borrow::new(self.raw.borrow(), self.value.get())
    }

    pub fn try_borrow(&self) -> Option<Ref<'_, T>> {
        if self.raw.try_borrow() {
            Some(Ref {
                value: self.value.get(),
                lock: &self.raw,
            })
        } else {
            None
        }
    }

    // /// Mutably borrows the wrapped value.
    // /// await for the resource availability if needed.
    //pub async fn borrow_mut(&self) -> RefMut<'_, T> {}
}

impl<T: fmt::Debug + ?Sized> fmt::Debug for RefCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Locked;
        impl fmt::Debug for Locked {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("<locked>")
            }
        }

        match self.try_borrow() {
            None => f.debug_struct("RefCell").field("value", &Locked).finish(),
            Some(guard) => f.debug_struct("RefCell").field("value", &&*guard).finish(),
        }
    }
}
pub struct Borrow<'b, T: ?Sized> {
    value: NonNull<T>,
    raw: RawBorrow<'b>, // &'b
                        //waker: Waker,
}

impl<'x, T: ?Sized> Borrow<'x, T> {
    fn new(raw: RawBorrow<'x>, value: *mut T) -> Self {
        let value = unsafe { NonNull::new_unchecked(value) };
        Self { value, raw }
    }
}

impl<'b, T: ?Sized + 'b> Future for Borrow<'b, T> {
    type Output = Ref<'b, T>;

    fn poll(
        mut self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> Poll<Self::Output> {
        if self.raw.try_borrow() {
            Poll::Ready(Ref::<T> {
                lock: self.raw.lock,
                value: self.value.as_ptr(),
            })
        } else {
            Poll::Pending
        }
    }
}

impl<T: ?Sized> fmt::Debug for Borrow<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Borrow { .. }")
    }
}

pub struct Ref<'a, T: ?Sized + 'a> {
    lock: &'a RawRefCell,
    value: *const T,
}

impl<T: ?Sized> Drop for Ref<'_, T> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: we are dropping a read guard.
        //unsafe {
        self.lock.borrow_unlock();
        //}
    }
}
impl<T: fmt::Debug + ?Sized> fmt::Debug for Ref<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}
impl<T: fmt::Display + ?Sized> fmt::Display for Ref<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}
impl<T: ?Sized> Deref for Ref<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { &*self.value }
    }
}

// //pub struct RefMut<'b, T: ?Sized + 'b> {}
