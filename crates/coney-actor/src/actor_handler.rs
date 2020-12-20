use std::error::Error as StdError;
use std::fmt;

use crate::actor_failure::ActorFailure;
use crate::context::Context;

#[async_trait::async_trait]
pub trait ActorHandler: fmt::Debug + Send + Sync + 'static {
    type State: fmt::Debug + Send;
    type Query: fmt::Debug + Send;
    type Value: fmt::Debug + Send + Sync;
    type Error: StdError + Send + Sync + 'static;

    async fn start(
        &mut self,
        _ctx: &mut Context<Self::Query>,
    ) -> Result<StartHandled<Self::State, Self::Value>, Self::Error>;

    async fn post_start(
        &mut self,
        _state: &mut Self::State,
        _ctx: &mut Context<Self::Query>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn handle_query(
        &mut self,
        state: &mut Self::State,
        ctx: &mut Context<Self::Query>,
        query: Self::Query,
    ) -> Result<QueryHandled<Self::Value>, Self::Error>;

    async fn pre_stop(
        &mut self,
        _state: &mut Self::State,
        _ctx: &mut Context<Self::Query>,
        _result: Result<&mut Self::Value, &ActorFailure<Self::Error>>,
    ) {
    }
    async fn post_stop(
        &mut self,
        _state: &mut Self::State,
        _result: Result<&mut Self::Value, &ActorFailure<Self::Error>>,
    ) {
    }
}

#[derive(Debug)]
pub enum StartHandled<State, Value> {
    Done(Value),
    Proceed(State),
}

#[derive(Debug)]
pub enum QueryHandled<Value> {
    Done(Value),
    Continue,
}
