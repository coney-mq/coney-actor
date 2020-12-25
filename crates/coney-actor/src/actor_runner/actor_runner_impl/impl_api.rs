use crate::actor::Actor;

use super::ActorRunner;

use super::api::Api;

impl<A> ActorRunner<A>
where
    A: Actor,
{
    pub fn api(&self) -> Api<A::Query> {
        Api {
            channels: self.client_channels.to_owned(),
        }
    }
}
