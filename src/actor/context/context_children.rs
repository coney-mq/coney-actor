use super::*;

use std::collections::HashMap;

use ::futures::lock::Mutex;

impl<Q> Context<Q> {
    pub fn children(&self) -> &Mutex<HashMap<usize, ChildHandle>> {
        self.children.as_ref()
    }
}
