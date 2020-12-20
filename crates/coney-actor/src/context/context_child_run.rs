use ::futures::channel::oneshot;

use crate::actor::Actor;
use crate::actor_ext::ActorExt;
use crate::actor_failure::ActorFailure;
use crate::actor_handler::ActorHandler;

use super::ChildHandle;
use super::Context;

#[derive(Debug)]
pub struct ChildSpawned<H: ActorHandler> {
    child_id: usize,
    child_output_rx: oneshot::Receiver<(Actor<H>, Result<H::Value, ActorFailure<H::Error>>)>,
}

impl<Q> Context<Q> {
    pub async fn child_run<H>(
        &mut self,
        mut child: Actor<H>,
        name: Option<String>,
    ) -> ChildSpawned<H>
    where
        H: ActorHandler + 'static,
    {
        let child_id = self.next_child_id;
        self.next_child_id += 1;

        let (child_output_tx, child_output_rx) = oneshot::channel();

        let child_spawned = ChildSpawned {
            child_id,
            child_output_rx,
        };

        let child_handle = ChildHandle {
            name,
            system_tx: child.chans.system_tx.to_owned(),
        };

        assert!(self
            .children
            .lock()
            .await
            .insert(child_id, child_handle)
            .is_none());

        let child_running = {
            let children = self.children.to_owned();
            async move {
                let actor_result = child.run().await;
                let _ = children.lock().await.remove(&child_id);
                let _ = child_output_tx.send((child, actor_result));
            }
        };

        let _ = self.future_spawn(child_running);

        child_spawned
    }
}

impl<H: ActorHandler> ChildSpawned<H> {
    pub fn id(&self) -> usize {
        self.child_id
    }

    // pub async fn value(self) -> (Actor<H>, Result<H::Value, ActorFailure<H::Error>> ){
    //     self.child_output_rx
    //         .await
    //         .map_err(|_| ActorFailure::OneshotGone)
    //         .and_then(|(a, r)| r)
    // }
}
