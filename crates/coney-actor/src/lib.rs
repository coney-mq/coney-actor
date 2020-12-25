#[macro_use]
extern crate thiserror;

pub mod actor;
pub mod actor_runner;
pub mod context;

pub mod prelude {
    pub use crate::actor::Actor;
    pub use crate::actor_runner::actor_ext::ActorExt;
    pub use crate::actor_runner::api::Api as ActorApi;
    pub use crate::context::Context;
}

#[cfg(test)]
mod tests;
