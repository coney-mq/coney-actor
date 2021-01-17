use std::error::Error as StdError;

use crate::context::Context;

#[async_trait::async_trait]
pub trait Actor: Send + 'static {
    type State: Send + Sync + 'static;
    type Error: StdError + Send + Sync + 'static;
    type Query: Send + Sync + 'static;

    async fn init<Ctx: Context>(&mut self, context: &mut Ctx) -> Result<Self::State, Self::Error>;

    async fn shutdown<Ctx: Context>(
        &mut self,
        context: &mut Ctx,
        state: Self::State,
        failure: Option<&Self::Error>,
    ) {
        let _ = (context, state, failure);
    }
}
