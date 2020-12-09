use super::*;

#[derive(Debug, Clone)]
pub struct ActorApi<Q> {
    pub(crate) query_tx: mpsc::UnboundedSender<Q>,
    pub(crate) system_tx: mpsc::UnboundedSender<SystemMessage>,
}
