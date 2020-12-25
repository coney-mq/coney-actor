use std::error::Error as StdError;

use ::futures::channel::mpsc;
use ::futures::channel::oneshot;
use ::futures::prelude::*;

use super::system_message::Shutdown;
use super::ClientChannels;
use super::ShutdownReason;
use super::SystemMessage;

#[derive(Debug)]
pub struct Api<Query> {
    pub(crate) channels: ClientChannels<Query>,
}
#[derive(Debug, Error)]
pub enum InvokeError<E: StdError + Send + Sync + 'static> {
    #[error("InvokeError::Specific")]
    Specific(#[source] E),

    #[error("InvokeError::ChanSend")]
    ChanSend(#[source] mpsc::SendError),

    #[error("InvokeError::ChanRecv")]
    ChanRecv(#[source] oneshot::Canceled),
}
pub type InvokeResult<T, E> = Result<T, InvokeError<E>>;

impl<Query> Api<Query> {
    pub async fn tell(&mut self, query: Query) -> InvokeResult<(), std::convert::Infallible> {
        let () = self
            .channels
            .normal_messages
            .send(query)
            .await
            .map_err(InvokeError::ChanSend)?;
        Ok(())
    }

    pub async fn shutdown(
        &mut self,
        reason: ShutdownReason,
    ) -> InvokeResult<(), std::convert::Infallible> {
        let (reply_tx, reply_rx) = oneshot::channel();
        let shutdown = Shutdown { reason, reply_tx };
        let system_message = SystemMessage::Shutdown(shutdown);

        let () = self
            .channels
            .system_messages
            .send(system_message)
            .await
            .map_err(InvokeError::ChanSend)?;
        let () = reply_rx.await.map_err(InvokeError::ChanRecv)?;

        Ok(())
    }
}
