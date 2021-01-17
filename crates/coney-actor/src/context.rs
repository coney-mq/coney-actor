use crate::actor_arena::ActorArena;

#[async_trait::async_trait]
pub trait Context: ActorArena + Send + Sync + 'static {}
