use super::*;

impl<Q> ActorApi<Q> {
    pub async fn shutdown(&mut self) {
        if let Err(tx_err) = self
            .system_tx
            .send(SystemMessage::Shutdown(Default::default()))
            .await
        {
            unimplemented!("emit a warning here: {:?}", tx_err)
        }
    }
}
