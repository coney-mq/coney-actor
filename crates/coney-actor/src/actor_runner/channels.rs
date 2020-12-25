use ::futures::channel::mpsc;

use super::SystemMessage;

#[derive(Debug)]
pub struct ServerChannels<Query> {
    pub(crate) system_messages: mpsc::UnboundedReceiver<SystemMessage>,
    pub(crate) normal_messages: mpsc::UnboundedReceiver<Query>,
}

#[derive(Debug)]
pub struct ClientChannels<Query> {
    pub(crate) system_messages: mpsc::UnboundedSender<SystemMessage>,
    pub(crate) normal_messages: mpsc::UnboundedSender<Query>,
}
impl<Query> Clone for ClientChannels<Query> {
    fn clone(&self) -> Self {
        Self {
            system_messages: self.system_messages.clone(),
            normal_messages: self.normal_messages.clone(),
        }
    }
}

pub fn create<Query>() -> (ClientChannels<Query>, ServerChannels<Query>) {
    let (stx, srx) = mpsc::unbounded();
    let (ntx, nrx) = mpsc::unbounded();

    let client = ClientChannels {
        system_messages: stx,
        normal_messages: ntx,
    };
    let server = ServerChannels {
        system_messages: srx,
        normal_messages: nrx,
    };

    (client, server)
}
