use super::ActorFailure;
use super::ActorHandler;
use super::Context;
use super::QueryHandled;

use super::AndThen;

pub async fn handle_query<H>(
    handler: &mut H,
    state: &mut H::State,
    ctx: &mut Context<H::Query>,
    query: H::Query,
) -> AndThen<(), Result<H::Value, ActorFailure<H::Error>>>
where
    H: ActorHandler,
{
    match handler.handle_query(state, ctx, query).await {
        Err(reason) => AndThen::Return(Err(ActorFailure::HandlerError(reason))),
        Ok(QueryHandled::Continue) => AndThen::Proceed(()),
        Ok(QueryHandled::Done(value)) => AndThen::Return(Ok(value)),
    }
}
