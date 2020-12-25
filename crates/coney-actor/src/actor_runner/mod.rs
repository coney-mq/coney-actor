use crate::actor::Actor;

pub mod actor_ext;
pub mod api;

mod context;
use context::ActorContext;

mod channels;
use channels::ClientChannels;
use channels::ServerChannels;

mod system_message;
pub use system_message::ShutdownReason;
use system_message::SystemMessage;

mod actor_runner_impl;

#[derive(Debug)]
pub struct ActorRunner<A: Actor> {
    actor: A,
    client_channels: ClientChannels<A::Query>,
    server_channels: ServerChannels<A::Query>,
}
