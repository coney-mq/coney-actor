use crate::actor_handler::ActorHandler;
use crate::chans::Chans;

#[derive(Debug)]
pub struct Actor<H: ActorHandler> {
    pub(crate) chans: Chans<H::Query>,
    pub(crate) handler: H,
}

impl<H: ActorHandler> Actor<H> {
    pub fn create(handler: H) -> Self {
        let chans = Chans::create();
        Self { chans, handler }
    }
}
