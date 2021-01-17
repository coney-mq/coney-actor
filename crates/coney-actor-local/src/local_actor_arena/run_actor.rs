use ::anyhow::Error as AnyError;
use ::futures::prelude::*;

use ::coney_actor::prelude::*;

use super::Command;
use super::Event;
use super::LocalActorContext;

pub async fn run_actor<A: Actor, CIn, EOut>(_actor: A, commands_inlet: CIn, events_outlet: EOut)
where
    CIn: Stream<Item = Command>,
    EOut: Sink<Event<A::Error>>,
{
    ::futures::pin_mut!(commands_inlet);

    // while let Some(command) = commands_inlet.next().await {
    //     let command = command.map_err(Into::into).map_err(Failure::InletFailure)?;
    // }

    unimplemented!()
}

fn create_context() -> impl Context {
    LocalActorContext {}
}
