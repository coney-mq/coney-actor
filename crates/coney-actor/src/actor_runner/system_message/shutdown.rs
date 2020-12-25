use ::futures::channel::oneshot;

#[derive(Debug)]
pub struct Shutdown {
    pub(crate) reason: ShutdownReason,
    pub(crate) reply_tx: oneshot::Sender<()>,
}

#[derive(Debug)]
pub enum ShutdownReason {
    Normal,
}
impl Default for ShutdownReason {
    fn default() -> Self {
        Self::Normal
    }
}
