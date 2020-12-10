#[macro_use]
extern crate thiserror;

pub mod actor;
pub mod context;

pub mod actor_api;
pub mod actor_ext;
pub mod actor_failure;
pub mod actor_handler;

mod actor_run;
mod chans;
mod system_message;

pub mod prelude {
    pub use crate::actor::Actor;
    pub use crate::actor_api::ActorApi;
    pub use crate::actor_ext::ActorExt;
    pub use crate::actor_failure::ActorFailure;
    pub use crate::actor_handler::ActorHandler;
    pub use crate::actor_handler::QueryHandled;
    pub use crate::actor_handler::StartHandled;
    pub use crate::context::Context;
}

#[cfg(test)]
mod tests;
