use super::system_message::ShutdownReason;

#[derive(Debug, Error)]
pub enum ActorFailure<E>
where
    E: std::error::Error + 'static,
{
    #[error("ActorFailure::HandlerError")]
    HandlerError(#[source] E),

    #[error("ActorFailure::OneshotGone")]
    OneshotGone,

    #[error("ActorFailure::Terminated")]
    Terminated(ShutdownReason),

    #[error("ActorFailure::UnexpectedRxTermination,")]
    UnexpectedRxTermination,
}
