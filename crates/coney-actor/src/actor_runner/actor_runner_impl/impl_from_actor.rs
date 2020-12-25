use crate::actor::Actor;

use super::channels;

use super::ActorRunner;

impl<A> From<A> for ActorRunner<A>
where
    A: Actor,
{
    fn from(actor: A) -> Self {
        let (client_channels, server_channels) = channels::create();
        Self {
            actor,
            server_channels,
            client_channels,
        }
    }
}
