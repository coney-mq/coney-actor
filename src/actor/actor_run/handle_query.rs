use super::ActorContext;
use super::ActorFailure;
use super::ActorHandler;
use super::QueryHandled;

use super::AndThen;

pub async fn handle_query<H>(
    handler: &mut H,
    ctx: &mut ActorContext<H::Query>,
    query: H::Query,
) -> Result<AndThen<H::Value>, ActorFailure<H::Error>>
where
    H: ActorHandler,
{
    match handler.handle_query(ctx, query).await {
        Err(reason) => Err(ActorFailure::HandlerError(reason)),
        Ok(QueryHandled::Continue) => Ok(AndThen::Continue),
        Ok(QueryHandled::Done(value)) => Ok(AndThen::Done(value)),
    }
}
