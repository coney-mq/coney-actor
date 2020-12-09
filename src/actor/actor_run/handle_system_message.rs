use super::ActorContext;
use super::ActorFailure;
use super::ActorHandler;
use super::AndThen;
use super::SystemMessage;

pub async fn handle_system_message<H>(
    _handler: &mut H,
    _context: &mut ActorContext<H::Query>,
    system_message: SystemMessage,
) -> Result<AndThen<H::Value>, ActorFailure<H::Error>>
where
    H: ActorHandler,
{
    match system_message {
        SystemMessage::Shutdown => Err(ActorFailure::Terminated),
    }
}
