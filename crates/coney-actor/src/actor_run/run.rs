use ::futures::prelude::*;

use crate::actor::Actor;
use crate::actor_failure::ActorFailure;
use crate::actor_handler::ActorHandler;
use crate::context::Context;

use super::AndThen;
use super::Event;

use super::process_shutdown;
use super::process_startup;
use super::run_event_loop;

pub async fn run<H: ActorHandler>(actor: Actor<H>) -> Result<H::Value, ActorFailure<H::Error>> {
    let mut handler = actor.handler;

    let system_tx = actor.chans.system_tx;
    let query_tx = actor.chans.query_tx;

    let mut ctx = Context::create(query_tx, system_tx);

    let mut state = match process_startup(&mut handler, &mut ctx).await {
        AndThen::Return(ret_value) => return ret_value,
        AndThen::Proceed(state) => state,
    };

    let system_rx = actor.chans.system_rx;
    let query_rx = actor.chans.query_rx;
    let system_rx = system_rx
        .map(Event::<H::Query>::SystemMessage)
        .chain(stream::once(async { Event::<H::Query>::SystemChanClosed }));
    let query_rx = query_rx
        .map(Event::Query)
        .chain(stream::once(async { Event::QueryChanClosed }));
    let events = stream::select(system_rx, query_rx);

    let loop_result = run_event_loop(events, &mut handler, &mut state, &mut ctx).await;

    process_shutdown(&mut handler, &mut ctx, &mut state, loop_result).await
}
