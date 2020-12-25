use super::*;

impl<A> ActorRunner<A>
where
    A: Actor,
{
    pub fn into_inner(self) -> A {
        self.actor
    }
}
