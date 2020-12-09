use super::ActorApi;

#[derive(Debug)]
pub struct ActorContext<Q> {
    pub(crate) self_api: ActorApi<Q>,
}
