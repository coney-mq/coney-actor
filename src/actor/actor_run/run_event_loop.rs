use ::futures::prelude::*;

use super::ActorContext;
use super::ActorFailure;
use super::ActorHandler;

use super::AndThen;
use super::Event;

use super::handle_query;
use super::handle_system_message;

pub async fn run_event_loop<E, H>(
    events: E,
    handler: &mut H,
    ctx: &mut ActorContext<H::Query>,
) -> Result<H::Value, ActorFailure<H::Error>>
where
    E: Stream<Item = Event<H::Query>>,
    H: ActorHandler,
{
    ::futures::pin_mut!(events);

    while let Some(event) = events.next().await {
        let and_then = match event {
            Event::SystemChanClosed => Err(ActorFailure::UnexpectedRxTermination)?,
            Event::QueryChanClosed => Err(ActorFailure::UnexpectedRxTermination)?,
            Event::SystemMessage(system_message) => {
                handle_system_message(handler, ctx, system_message).await?
            }
            Event::Query(query) => handle_query(handler, ctx, query).await?,
        };
        match and_then {
            AndThen::Continue => continue,
            AndThen::Done(value) => return Ok(value),
        }
    }

    Err(ActorFailure::UnexpectedRxTermination)
}
