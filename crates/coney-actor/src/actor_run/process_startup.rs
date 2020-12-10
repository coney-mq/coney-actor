use crate::actor_failure::ActorFailure;
use crate::actor_handler::ActorHandler;
use crate::actor_handler::StartHandled;
use crate::context::Context;

use super::process_shutdown;
use super::AndThen;

pub async fn process_startup<H>(
    handler: &mut H,
    ctx: &mut Context<H::Query>,
) -> AndThen<H::State, Result<H::Value, ActorFailure<H::Error>>>
where
    H: ActorHandler,
{
    match handler.start(ctx).await {
        Ok(StartHandled::Proceed(mut state)) => match handler.post_start(&mut state, ctx).await {
            Ok(()) => AndThen::Proceed(state),
            Err(reason) => {
                let ret_value = process_shutdown(
                    handler,
                    ctx,
                    &mut state,
                    Err(ActorFailure::HandlerError(reason)),
                )
                .await;
                AndThen::Return(ret_value)
            }
        },

        Ok(StartHandled::Done(value)) => AndThen::Return(Ok(value)),

        Err(reason) => AndThen::Return(Err(ActorFailure::HandlerError(reason))),
    }
}
