#[derive(Debug)]
pub enum Event<E> {
    InitOk,
    InitFailure(E),
    RunFailure(E),
    NormalTermination,
}
