use ::futures::channel::oneshot;
use ::futures::prelude::*;

#[derive(Debug)]
pub struct Shutdown {
    pub(crate) reply_tx: oneshot::Sender<()>,
    pub(crate) reason: ShutdownReason,
}

#[derive(Debug)]
pub enum ShutdownReason {
    Normal,
    ParentTerminated,
}

impl Shutdown {
    pub fn new(reason: ShutdownReason) -> (Self, impl Future<Output = ()>) {
        let (reply_tx, reply_rx) = oneshot::channel();

        (Self { reply_tx, reason }, reply_rx.map(|_| ()))
    }
}

impl Default for ShutdownReason {
    fn default() -> Self {
        Self::Normal
    }
}
