#[macro_use]
extern crate thiserror;

pub mod context;
pub use context::Context;

mod actor;
pub use actor::Actor;

mod actor_api;
pub use actor_api::ActorApi;

mod actor_handler;
pub use actor_handler::ActorHandler;
pub use actor_handler::QueryHandled;
pub use actor_handler::StartHandled;

mod actor_ext;
pub use actor_ext::ActorExt;

mod actor_failure;
pub use actor_failure::ActorFailure;

mod chans;
use chans::Chans;

mod system_message;
use system_message::SystemMessage;
mod actor_run;

pub mod prelude {
    pub use crate::Context;

    pub use crate::Actor;
    pub use crate::ActorApi;
    pub use crate::ActorExt;
    pub use crate::ActorFailure;

    pub use crate::ActorHandler;
    pub use crate::QueryHandled;
    pub use crate::StartHandled;
}

#[cfg(test)]
mod tests;
