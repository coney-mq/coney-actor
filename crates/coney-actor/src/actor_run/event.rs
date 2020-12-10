use crate::system_message::SystemMessage;

#[derive(Debug)]
pub enum Event<Q> {
    SystemChanClosed,
    QueryChanClosed,
    SystemMessage(SystemMessage),
    Query(Q),
}
