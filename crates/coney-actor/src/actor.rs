use std::error::Error as StdError;
use std::fmt;

use crate::context::Context;

#[async_trait::async_trait]
pub trait Actor: fmt::Debug + Sized + Send + Sync {
    type Query: fmt::Debug + Send + Sync + 'static;
    type State: fmt::Debug + Send + Sync;
    type Error: StdError + Send + Sync + 'static;

    async fn init<Ctx: Context>(&mut self, context: &mut Ctx) -> Result<Self::State, Self::Error>;

    async fn handle_query<Ctx: Context>(
        state: &mut Self::State,
        context: &mut Ctx,
        query: Self::Query,
    ) -> Result<(), Self::Error>;

    async fn shutdown<Ctx: Context>(
        &mut self,
        _state: Self::State,
        _context: &mut Ctx,
        _failure: Option<&Self::Error>,
    ) {
    }
}
