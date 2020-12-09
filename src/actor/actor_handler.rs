use std::error::Error as StdError;

use super::ActorFailure;
use super::Context;

#[async_trait::async_trait]
pub trait ActorHandler: Send + Sync {
    type Query: Send;
    type Value: Send + Sync;
    type Error: StdError + Send + Sync + 'static;

    async fn on_start(
        &mut self,
        _ctx: &mut Context<Self::Query>,
    ) -> Result<StartHandled<Self::Value>, Self::Error> {
        Ok(StartHandled::Proceed)
    }

    async fn handle_query(
        &mut self,
        ctx: &mut Context<Self::Query>,
        query: Self::Query,
    ) -> Result<QueryHandled<Self::Value>, Self::Error>;

    async fn on_complete(&mut self, _ctx: &mut Context<Self::Query>, _value: &mut Self::Value) {}
    async fn on_failure(
        &mut self,
        _ctx: &mut Context<Self::Query>,
        _reason: &ActorFailure<Self::Error>,
    ) {
    }
}

#[derive(Debug)]
pub enum StartHandled<Value> {
    Done(Value),
    Proceed,
}

#[derive(Debug)]
pub enum QueryHandled<Value> {
    Done(Value),
    Continue,
}
