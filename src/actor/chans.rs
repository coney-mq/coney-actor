use ::futures::channel::mpsc;

use super::SystemMessage;

#[derive(Debug)]
pub struct Chans<Q> {
    pub(crate) query_tx: mpsc::UnboundedSender<Q>,
    pub(crate) query_rx: mpsc::UnboundedReceiver<Q>,
    pub(crate) system_tx: mpsc::UnboundedSender<SystemMessage>,
    pub(crate) system_rx: mpsc::UnboundedReceiver<SystemMessage>,
}

impl<Q> Chans<Q> {
    pub fn create() -> Self {
        let (query_tx, query_rx) = mpsc::unbounded();
        let (system_tx, system_rx) = mpsc::unbounded();
        Self {
            query_tx,
            query_rx,
            system_tx,
            system_rx,
        }
    }
}
