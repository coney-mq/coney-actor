use super::system_message as sm;
use super::ActorFailure;
use super::ActorHandler;
use super::AndThen;
use super::Context;
use super::SystemMessage;

pub async fn handle_system_message<H>(
    handler: &mut H,
    context: &mut Context<H::Query>,
    system_message: SystemMessage,
) -> AndThen<(), Result<H::Value, ActorFailure<H::Error>>>
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
) -> AndThen<(), Result<H::Value, ActorFailure<H::Error>>>
where
    H: ActorHandler,
{
    let () = context.shutdown_notifications.push(request.reply_tx);
    AndThen::Return(Err(ActorFailure::Terminated(request.reason)))
}

async fn handle_get_children<H>(
    _handler: &mut H,
    context: &mut Context<H::Query>,
    request: sm::GetChildren,
) -> AndThen<(), Result<H::Value, ActorFailure<H::Error>>>
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

    AndThen::Proceed(())
}
