use super::system_message as sm;
use super::ActorFailure;
use super::ActorHandler;
use super::AndThen;
use super::Context;
use super::SystemMessage;

use ::futures::prelude::*;

pub async fn handle_system_message<H>(
    handler: &mut H,
    context: &mut Context<H::Query>,
    system_message: SystemMessage,
) -> Result<AndThen<H::Value>, ActorFailure<H::Error>>
where
    H: ActorHandler,
{
    match system_message {
        SystemMessage::Shutdown(shutdown) => handle_shutdown(handler, context, shutdown).await,
    }
}

async fn handle_shutdown<H>(
    _handler: &mut H,
    context: &mut Context<H::Query>,
    _shutdown: sm::Shutdown,
) -> Result<AndThen<H::Value>, ActorFailure<H::Error>>
where
    H: ActorHandler,
{
    let mut children = context.children().lock().await;
    for (_child_id, child_handle) in children.iter_mut() {
        let _ = child_handle
            .system_tx_mut()
            .send(SystemMessage::Shutdown(Default::default()))
            .await;
    }
    Err(ActorFailure::Terminated)
}
