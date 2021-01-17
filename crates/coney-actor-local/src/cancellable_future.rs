use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use ::futures::channel::oneshot;
use ::futures::prelude::*;

pub trait CancellableExt: Future + Sized {
    fn cancellable(self) -> (KillSwitch, Cancellable<Self>) {
        let (prolonged_tx, prolonged_rx) = oneshot::channel();
        let kill_switch = KillSwitch { prolonged_tx };
        let cancellable = Cancellable {
            prolonged: false,
            prolonged_rx,
            inner: self,
        };
        (kill_switch, cancellable)
    }
}

#[derive(Debug)]
pub struct KillSwitch {
    prolonged_tx: oneshot::Sender<bool>,
}

impl KillSwitch {
    pub fn prolong(self) {
        let _ = self.prolonged_tx.send(true);
    }
    pub fn cancel(self) {
        let _ = self.prolonged_tx.send(false);
    }
}

#[pin_project]
#[derive(Debug)]
pub struct Cancellable<F> {
    #[pin]
    prolonged: bool,

    #[pin]
    prolonged_rx: oneshot::Receiver<bool>,

    #[pin]
    inner: F,
}

impl<F> CancellableExt for F where F: Future + Sized {}

impl<F> Future for Cancellable<F>
where
    F: Future,
{
    type Output = Option<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.prolonged {
            self.project().inner.poll(cx).map(Some)
        } else {
            let mut this = self.project();
            match this.prolonged_rx.poll(cx) {
                Poll::Pending => this.inner.poll(cx).map(Some),
                Poll::Ready(Ok(true)) => {
                    *this.prolonged = true;
                    this.inner.poll(cx).map(Some)
                }
                Poll::Ready(_) => Poll::Ready(None),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn normal_run() {
        use std::time::Duration;
        let in_a_second = ::tokio::time::sleep(Duration::from_millis(100));
        let () = in_a_second.await;
    }

    #[tokio::test]
    async fn not_cancelled() {
        use std::time::Duration;
        let (_kill_switch, in_a_second) =
            ::tokio::time::sleep(Duration::from_millis(100)).cancellable();
        assert_eq!(in_a_second.await, Some(()));
    }

    #[tokio::test]
    async fn cancelled_on_drop() {
        use std::time::Duration;
        let (kill_switch, in_a_second) =
            ::tokio::time::sleep(Duration::from_millis(100)).cancellable();
        let () = std::mem::drop(kill_switch);
        assert_eq!(in_a_second.await, None);
    }

    #[tokio::test]
    async fn cancelled_explicitly() {
        use std::time::Duration;
        let (kill_switch, in_a_second) =
            ::tokio::time::sleep(Duration::from_millis(100)).cancellable();
        let () = kill_switch.cancel();
        assert_eq!(in_a_second.await, None);
    }

    #[tokio::test]
    async fn prolonged() {
        use std::time::Duration;
        let (kill_switch, in_a_second) =
            ::tokio::time::sleep(Duration::from_millis(100)).cancellable();
        let () = kill_switch.prolong();
        assert_eq!(in_a_second.await, Some(()));
    }
}
