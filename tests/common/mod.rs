use std::io::{self, Result};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use async_io::Timer;
use futures_lite::{future, prelude::*};
use pin_project_lite::pin_project;
use waker_fn::waker_fn;

pub fn check_yields_when_contended<G>(contending_guard: G, acquire_future: impl Future) {
    let was_woken = Arc::new(AtomicBool::new(false));
    let waker = {
        let was_woken = Arc::clone(&was_woken);
        waker_fn(move || was_woken.store(true, Ordering::SeqCst))
    };
    let mut cx = Context::from_waker(&waker);

    futures_lite::pin!(acquire_future);
    assert!(acquire_future.as_mut().poll(&mut cx).is_pending());
    drop(contending_guard);
    assert!(was_woken.load(Ordering::SeqCst));
    assert!(acquire_future.poll(&mut cx).is_ready());
}

pub async fn sleep(dur: Duration) {
    let _: Result<()> = timeout(dur, future::pending()).await;
}

mod timer {
    pub type Timer = async_io::Timer;
}
pub(crate) fn timer_after(dur: std::time::Duration) -> timer::Timer {
    Timer::after(dur)
}

pub async fn timeout<F, T>(dur: Duration, f: F) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    Timeout {
        timeout: timer_after(dur),
        future: f,
    }
    .await
}

pin_project! {
    /// Future returned by the `FutureExt::timeout` method.
    #[derive(Debug)]
    pub struct Timeout<F, T>
    where
        F: Future<Output = Result<T>>,
    {
        #[pin]
        future: F,
        #[pin]
        timeout: Timer,
    }
}

impl<F, T> Future for Timeout<F, T>
where
    F: Future<Output = Result<T>>,
{
    type Output = Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.future.poll(cx) {
            Poll::Pending => {}
            other => return other,
        }

        if this.timeout.poll(cx).is_ready() {
            let err = Err(io::Error::new(io::ErrorKind::TimedOut, "future timed out"));
            Poll::Ready(err)
        } else {
            Poll::Pending
        }
    }
}
