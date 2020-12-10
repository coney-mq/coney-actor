#[derive(Debug)]
pub enum AndThen<Proceed, Return> {
    Proceed(Proceed),
    Return(Return),
}
