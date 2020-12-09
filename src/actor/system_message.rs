use ::futures::channel::mpsc;
use ::futures::channel::oneshot;
use ::futures::prelude::*;

#[derive(Debug)]
pub enum SystemMessage {
    Shutdown(Shutdown),
    GetChildren(GetChildren),
}

#[derive(Debug)]
pub struct GetChildren {
    pub(crate) reply_tx:
        oneshot::Sender<Vec<(usize, Option<String>, mpsc::UnboundedSender<SystemMessage>)>>,
}

#[derive(Debug)]
pub struct Shutdown {
    pub(crate) reply_tx: oneshot::Sender<()>,
}

impl GetChildren {
    pub fn new() -> (
        Self,
        impl Future<Output = Vec<(usize, Option<String>, mpsc::UnboundedSender<SystemMessage>)>>,
    ) {
        let (reply_tx, reply_rx) = oneshot::channel();

        (Self { reply_tx }, reply_rx.map(|r| r.unwrap_or_default()))
    }
}

impl Shutdown {
    pub fn new() -> (Self, impl Future<Output = ()>) {
        let (reply_tx, reply_rx) = oneshot::channel();

        (Self { reply_tx }, reply_rx.map(|_| ()))
    }
}
