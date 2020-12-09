use super::*;

use ::futures::prelude::*;

impl<Q> Context<Q> {
    pub fn future_spawn<T>(&mut self, task: T) -> impl Future<Output = T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        ::tokio::spawn(task).then(|result| future::ready(result.expect("Failed to spawn a future")))
    }
}
