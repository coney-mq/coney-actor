use ::coney_actor::prelude::*;

pub struct LocalActorContext {}

#[async_trait::async_trait]
impl ActorArena for LocalActorContext {
    async fn start<A: Actor>(
        &mut self,
        _actor: A,
    ) -> Result<Box<dyn ActorRef<Query = A::Query>>, A::Error> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl Context for LocalActorContext {}
