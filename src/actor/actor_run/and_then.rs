#[derive(Debug)]
pub enum AndThen<V> {
    Done(V),
    Continue,
}
