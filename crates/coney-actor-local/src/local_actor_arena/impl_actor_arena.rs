use ::futures::channel::mpsc;
use ::futures::prelude::*;

use ::coney_actor::prelude::*;

use crate::cancellable_future::CancellableExt;

use super::*;

#[async_trait::async_trait]
impl ActorArena for LocalActorArena {
    async fn start<A: Actor>(
        &mut self,
        actor: A,
    ) -> Result<Box<dyn ActorRef<Query = A::Query>>, A::Error> {
        let (events_tx, events_rx) = mpsc::unbounded();
        let (commands_tx, commands_rx) = mpsc::unbounded();

        let (kill_switch, actor_running) = run_actor(actor, commands_rx, events_tx).cancellable();
        ::tokio::spawn(actor_running);

        unimplemented!("<LocalActorArena as ActorArena>::start")
    }
}
