use super::*;

impl<Q> ActorApi<Q> {
    pub async fn shutdown(&mut self) {
        let (shutdown_rq, shutdown_rs) = system_message::Shutdown::new();
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
