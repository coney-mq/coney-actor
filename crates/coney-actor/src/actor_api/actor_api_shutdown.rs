use ::futures::prelude::*;

use crate::system_message::Shutdown;
use crate::system_message::ShutdownReason;
use crate::system_message::SystemMessage;

use super::ActorApi;

impl<Q> ActorApi<Q> {
    pub async fn shutdown(&mut self, reason: ShutdownReason) {
        let (shutdown_rq, shutdown_rs) = Shutdown::new(reason);
        if let Err(tx_err) = self
            .system_tx
            .send(SystemMessage::Shutdown(shutdown_rq))
            .await
        {
            eprintln!("emit a warning here: {:?}", tx_err)
        } else {
            shutdown_rs.await
        }
    }
}
