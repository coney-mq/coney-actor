use std::error::Error as StdError;

use crate::actor::Actor;
use crate::actor_api::ActorApi;
use crate::actor_failure::ActorFailure;
use crate::actor_handler::ActorHandler;

#[async_trait::async_trait]
pub trait ActorExt: Send + Sync {
    type Query: Send;
    type Value: Send;
    type Error: StdError + Sync + Send + 'static;

    fn api(&self) -> ActorApi<Self::Query>;
    async fn run(&mut self) -> Result<Self::Value, Self::Error>;
}

#[async_trait::async_trait]
impl<H> ActorExt for Actor<H>
where
    H: ActorHandler,
{
    type Query = H::Query;
    type Value = H::Value;
    type Error = ActorFailure<H::Error>;

    fn api(&self) -> ActorApi<Self::Query> {
        ActorApi {
            query_tx: self.chans.query_tx.to_owned(),
            system_tx: self.chans.system_tx.to_owned(),
        }
    }

    async fn run(&mut self) -> Result<Self::Value, Self::Error> {
        super::actor_run::run(self).await
    }
}
