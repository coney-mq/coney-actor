use std::error::Error as StdError;

use super::Actor;
use super::ActorApi;
use super::ActorFailure;
use super::ActorHandler;

#[async_trait::async_trait]
pub trait ActorExt: Send + Sync {
    type Query: Send;
    type Value: Send;
    type Error: StdError + Sync + Send + 'static;

    fn api(&self) -> ActorApi<Self::Query>;
    async fn run(self) -> Result<Self::Value, Self::Error>;
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

    async fn run(self) -> Result<Self::Value, Self::Error> {
        super::actor_run::run(self).await
    }
}
