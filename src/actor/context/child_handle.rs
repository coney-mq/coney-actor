use super::*;

use ::futures::channel::mpsc;

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
}
