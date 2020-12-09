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
        SystemMessage::GetChildren(get_children) => {
            handle_get_children(handler, context, get_children).await
        }
    }
}

async fn handle_shutdown<H>(
    _handler: &mut H,
    context: &mut Context<H::Query>,
    request: sm::Shutdown,
) -> Result<AndThen<H::Value>, ActorFailure<H::Error>>
where
    H: ActorHandler,
{
    let mut children = context.children().lock().await;
    for (_child_id, child_handle) in children.iter_mut() {
        let (shutdown_rq, shutdown_result) = sm::Shutdown::new();
        let _ = child_handle
            .system_tx_mut()
            .send(SystemMessage::Shutdown(shutdown_rq))
            .await;
        let () = shutdown_result.await;
    }
    let _ = request.reply_tx.send(());
    Err(ActorFailure::Terminated)
}

async fn handle_get_children<H>(
    _handler: &mut H,
    context: &mut Context<H::Query>,
    request: sm::GetChildren,
) -> Result<AndThen<H::Value>, ActorFailure<H::Error>>
where
    H: ActorHandler,
{
    let children = context
        .children()
        .lock()
        .await
        .iter()
        .map(|(id, ch)| {
            (
                *id,
                ch.name().map(|s| s.to_owned()),
                ch.system_tx().to_owned(),
            )
        })
        .collect();
    let _ = request.reply_tx.send(children);
    Err(ActorFailure::Terminated)
}
