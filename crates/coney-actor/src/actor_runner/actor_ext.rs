use crate::actor::Actor;

use super::ActorRunner;

pub trait ActorExt: Actor {
    fn into_runner(self) -> ActorRunner<Self> {
        self.into()
    }
}

impl<A> ActorExt for A where A: Actor {}
