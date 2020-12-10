use super::*;

use std::collections::HashMap;
use std::sync::Arc;

use ::futures::channel::mpsc;
use ::futures::channel::oneshot;
use ::futures::lock::Mutex;

#[derive(Debug)]
pub struct Context<Q> {
    pub(super) this_query_tx: mpsc::UnboundedSender<Q>,
    pub(super) this_system_tx: mpsc::UnboundedSender<SystemMessage>,

    pub(super) next_child_id: usize,
    pub(super) children: Arc<Mutex<HashMap<usize, ChildHandle>>>,

    pub(crate) shutdown_notifications: Vec<oneshot::Sender<()>>,
}
