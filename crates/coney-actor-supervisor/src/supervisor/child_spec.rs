
use crate::actor::Actor;
use crate::actor::ActorHandler;

pub struct ChildSpec<K, H: ActorHandler> {
    key: K,
    actor: Actor<H>,
}


