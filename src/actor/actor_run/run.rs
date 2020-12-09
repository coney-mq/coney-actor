use ::futures::prelude::*;

use super::Actor;
use super::ActorFailure;
use super::ActorHandler;
use super::Context;

use super::Event;
use super::StartHandled;

use super::run_event_loop;

pub async fn run<H: ActorHandler>(actor: Actor<H>) -> Result<H::Value, ActorFailure<H::Error>> {
    let mut handler = actor.handler;

    let system_tx = actor.chans.system_tx;
    let query_tx = actor.chans.query_tx;

    let mut ctx = Context::create(query_tx, system_tx);

    match handler.on_start(&mut ctx).await {
        Ok(StartHandled::Proceed) => {}

        Ok(StartHandled::Done(mut value)) => {
            let () = on_complete(&mut handler, &mut ctx, &mut value).await;
            return Ok(value);
        }

        Err(reason) => {
            let reason = ActorFailure::HandlerError(reason);
            let () = on_failure(&mut handler, &mut ctx, &reason).await;
            return Err(reason);
        }
    }

    let system_rx = actor.chans.system_rx;
    let query_rx = actor.chans.query_rx;
    let system_rx = system_rx
        .map(Event::<H::Query>::SystemMessage)
        .chain(stream::once(async { Event::<H::Query>::SystemChanClosed }));
    let query_rx = query_rx
        .map(Event::Query)
        .chain(stream::once(async { Event::QueryChanClosed }));
    let events = stream::select(system_rx, query_rx);

    let loop_result = run_event_loop(events, &mut handler, &mut ctx).await;

    match loop_result {
        Ok(mut value) => {
            let () = on_complete(&mut handler, &mut ctx, &mut value).await;
            Ok(value)
        }
        Err(reason) => {
            let () = on_failure(&mut handler, &mut ctx, &reason).await;
            Err(reason)
        }
    }
}

async fn on_complete<H: ActorHandler>(
    handler: &mut H,
    ctx: &mut Context<H::Query>,
    value: &mut H::Value,
) {
    handler.on_complete(ctx, value).await
}

async fn on_failure<H: ActorHandler>(
    handler: &mut H,
    ctx: &mut Context<H::Query>,
    reason: &ActorFailure<H::Error>,
) {
    handler.on_failure(ctx, reason).await
}
