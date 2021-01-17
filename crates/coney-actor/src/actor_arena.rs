use crate::actor::Actor;
use crate::actor_ref::ActorRef;

#[async_trait::async_trait]
pub trait ActorArena {
    async fn start<A: Actor>(
        &mut self,
        actor: A,
    ) -> Result<Box<dyn ActorRef<Query = A::Query>>, A::Error>;
}
