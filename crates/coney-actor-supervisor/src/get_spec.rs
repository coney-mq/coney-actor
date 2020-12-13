use crate::cons_spec::ConsSpec;
use crate::spec::Spec;

pub trait GetSpec<A, Idx> {
    fn get(&self) -> &Spec<A>;
    fn get_mut(&mut self) -> &mut Spec<A>;
}

#[derive(Debug)]
pub struct Here;

#[derive(Debug)]
pub struct There<T> {
    _pd: std::marker::PhantomData<T>,
}

impl<A> GetSpec<A, Here> for Spec<A> {
    fn get(&self) -> &Spec<A> {
        self
    }
    fn get_mut(&mut self) -> &mut Spec<A> {
        self
    }
}

impl<Head, Tail> GetSpec<Head, Here> for ConsSpec<Head, Tail> {
    fn get(&self) -> &Spec<Head> {
        &self.head
    }
    fn get_mut(&mut self) -> &mut Spec<Head> {
        &mut self.head
    }
}

impl<Head, Tail, TailIndex, A> GetSpec<A, There<TailIndex>> for ConsSpec<Head, Tail>
where
    Tail: GetSpec<A, TailIndex>,
{
    fn get(&self) -> &Spec<A> {
        self.tail.get()
    }
    fn get_mut(&mut self) -> &mut Spec<A> {
        self.tail.get_mut()
    }
}
