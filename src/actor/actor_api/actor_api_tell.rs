use super::*;

impl<Q> ActorApi<Q> {
    pub async fn tell<T: Into<Q>>(&mut self, q: T) {
        if let Err(tx_err) = self.query_tx.send(q.into()).await {
            unimplemented!("emit a warning here: {:?}", tx_err)
        }
    }
}
