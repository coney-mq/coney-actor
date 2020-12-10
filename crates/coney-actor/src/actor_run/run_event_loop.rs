use ::futures::prelude::*;

use crate::actor_failure::ActorFailure;
use crate::actor_handler::ActorHandler;
use crate::context::Context;

use super::AndThen;
use super::Event;

use super::handle_query;
use super::handle_system_message;

pub async fn run_event_loop<E, H>(
    events: E,
    handler: &mut H,
    state: &mut H::State,
    ctx: &mut Context<H::Query>,
) -> Result<H::Value, ActorFailure<H::Error>>
where
    E: Stream<Item = Event<H::Query>>,
    H: ActorHandler,
{
    ::futures::pin_mut!(events);

    while let Some(event) = events.next().await {
        let and_then = match event {
            Event::SystemChanClosed => AndThen::Return(Err(ActorFailure::UnexpectedRxTermination)),
            Event::QueryChanClosed => AndThen::Return(Err(ActorFailure::UnexpectedRxTermination)),
            Event::SystemMessage(system_message) => {
                handle_system_message(handler, ctx, system_message).await
            }
            Event::Query(query) => handle_query(handler, state, ctx, query).await,
        };
        match and_then {
            AndThen::Return(ret_value) => return ret_value,
            AndThen::Proceed(()) => continue,
        }
    }

    Err(ActorFailure::UnexpectedRxTermination)
}
