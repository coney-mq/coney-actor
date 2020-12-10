use ::futures::prelude::*;

use crate::actor_failure::ActorFailure;
use crate::actor_handler::ActorHandler;
use crate::context::Context;

use crate::system_message::Shutdown;
use crate::system_message::ShutdownReason;
use crate::system_message::SystemMessage;

pub async fn process_shutdown<H>(
    handler: &mut H,
    ctx: &mut Context<H::Query>,
    state: &mut H::State,
    mut result: Result<H::Value, ActorFailure<H::Error>>,
) -> Result<H::Value, ActorFailure<H::Error>>
where
    H: ActorHandler,
{
    let result_ref = match result {
        Ok(ref mut value) => Ok(value),
        Err(ref error) => Err(error),
    };
    let () = handler.pre_stop(state, ctx, result_ref).await;

    let mut children = ctx.children().lock().await;
    let mut children_shutting_down = Vec::new();
    for (_child_id, child_handle) in children.iter_mut() {
        let (shutdown_rq, shutdown_result) = Shutdown::new(ShutdownReason::ParentTerminated);
        let () = children_shutting_down.push(shutdown_result);

        let _ = child_handle
            .system_tx_mut()
            .send(SystemMessage::Shutdown(shutdown_rq))
            .await;
    }
    let _ = future::join_all(children_shutting_down).await;

    let result_ref = match result {
        Ok(ref mut value) => Ok(value),
        Err(ref error) => Err(error),
    };
    let () = handler.post_stop(state, result_ref).await;

    result
}
