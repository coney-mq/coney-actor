mod impl_context;

#[derive(Debug)]
pub struct ActorContext {}

impl ActorContext {
    pub fn new() -> Self {
        Self {}
    }

    pub(crate) async fn shutdown(self) {}
}
