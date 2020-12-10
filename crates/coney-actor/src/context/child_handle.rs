use ::futures::channel::mpsc;

use crate::system_message::SystemMessage;

#[derive(Debug)]
pub struct ChildHandle {
    pub(crate) name: Option<String>,
    pub(crate) system_tx: mpsc::UnboundedSender<SystemMessage>,
}

impl ChildHandle {
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_ref())
    }
    pub fn system_tx_mut(&mut self) -> &mut mpsc::UnboundedSender<SystemMessage> {
        &mut self.system_tx
    }
    pub fn system_tx(&self) -> &mpsc::UnboundedSender<SystemMessage> {
        &self.system_tx
    }
}
