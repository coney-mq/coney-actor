use super::*;

use ::futures::channel::mpsc;

impl<Q> Context<Q> {
    pub fn create(
        this_query_tx: mpsc::UnboundedSender<Q>,
        this_system_tx: mpsc::UnboundedSender<SystemMessage>,
    ) -> Self {
        Self {
            this_query_tx,
            this_system_tx,
            next_child_id: 0,
            children: Default::default(),
        }
    }
}
