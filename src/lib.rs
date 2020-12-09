#[macro_use]
extern crate thiserror;

pub mod actor;

pub mod prelude {
    pub use crate::actor::Context;

    pub use crate::actor::Actor;
    pub use crate::actor::ActorApi;
    pub use crate::actor::ActorExt;
    pub use crate::actor::ActorFailure;

    pub use crate::actor::ActorHandler;
    pub use crate::actor::QueryHandled;
    pub use crate::actor::StartHandled;
}

#[cfg(test)]
mod tests;
