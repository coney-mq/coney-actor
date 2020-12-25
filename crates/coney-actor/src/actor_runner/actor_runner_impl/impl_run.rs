use std::error::Error as StdError;

use ::futures::prelude::*;

use crate::{actor::Actor, actor_runner::system_message};

use super::ActorContext;
use super::ActorRunner;
use super::SystemMessage;

#[derive(Debug, Error)]
pub enum ActorRunFailure<ActorSpecific: StdError + Send + Sync + 'static> {
    #[error("Generic")]
    Generic(#[source] ActorGenericFailure),

    #[error("InitFailure")]
    InitFailure(#[source] ActorSpecific),

    #[error("RunFailure")]
    RunFailure(#[source] ActorSpecific),
}

#[derive(Debug, Error)]
pub enum ActorGenericFailure {
    #[error("ActorGenericFailure::SystemMessagesChannelClosed")]
    SystemMessagesChannelClosed,

    #[error("ActorGenericFailure::QueriesChannelClosed")]
    QueriesChannelClosed,
}

impl<A> ActorRunner<A>
where
    A: Actor,
{
    pub async fn run(&mut self) -> Result<(), ActorRunFailure<A::Error>> {
        let mut context = ActorContext::new();
        let mut state = A::init(&mut self.actor, &mut context)
            .await
            .map_err(ActorRunFailure::InitFailure)?;

        let mut next_system_message = self.server_channels.system_messages.next();
        let mut next_query = self.server_channels.normal_messages.next();

        loop {
            let and_then = ::futures::select! {
                system_message_opt = next_system_message => {
                    if let Some(system_message) = system_message_opt {
                        next_system_message = self.server_channels.system_messages.next();
                        process_system_message::<A>(&mut context, &mut state, system_message).await
                    } else {
                        Err(ActorGenericFailure::SystemMessagesChannelClosed).map_err(ActorRunFailure::Generic)?
                    }
                },
                next_query_opt = next_query => {
                    if let Some(query) = next_query_opt {
                        next_query = self.server_channels.normal_messages.next();
                        process_query::<A>(&mut context, &mut state, query).await
                    } else {
                        Err(ActorGenericFailure::QueriesChannelClosed).map_err(ActorRunFailure::Generic)?
                    }
                },
                complete => unimplemented!(),
            };
            match and_then {
                AndThen::Proceed => continue,
                AndThen::Shutdown => {
                    let () = self.actor.shutdown(state, &mut context, None).await;
                    let () = context.shutdown().await;
                    return Ok(());
                }
                AndThen::Fail(e) => {
                    let () = self.actor.shutdown(state, &mut context, Some(&e)).await;
                    let () = context.shutdown().await;
                    return Err(e).map_err(ActorRunFailure::RunFailure);
                }
            }
        }
    }
}

#[derive(Debug)]
enum AndThen<E: StdError + Send + Sync + 'static> {
    Proceed,
    Shutdown,
    Fail(E),
}

async fn process_system_message<A: Actor>(
    context: &mut ActorContext,
    state: &mut A::State,
    system_message: SystemMessage,
) -> AndThen<A::Error> {
    match system_message {
        SystemMessage::Shutdown(shutdown) => {
            process_system_message_shutdown::<A>(context, state, shutdown).await
        }
    }
}

async fn process_system_message_shutdown<A: Actor>(
    _context: &mut ActorContext,
    _state: &mut A::State,
    shutdown: system_message::Shutdown,
) -> AndThen<A::Error> {
    let _ = shutdown.reply_tx.send(());
    AndThen::Shutdown
}

async fn process_query<A: Actor>(
    context: &mut ActorContext,
    state: &mut A::State,
    query: A::Query,
) -> AndThen<A::Error> {
    match A::handle_query(state, context, query).await {
        Ok(()) => AndThen::Proceed,
        Err(reason) => AndThen::Fail(reason),
    }
}
