use ::futures::channel::mpsc;
use ::futures::channel::oneshot;
use ::futures::prelude::*;

use super::SystemMessage;

#[derive(Debug)]
pub struct GetChildren {
    pub(crate) reply_tx:
        oneshot::Sender<Vec<(usize, Option<String>, mpsc::UnboundedSender<SystemMessage>)>>,
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
