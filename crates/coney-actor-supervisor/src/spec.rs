use crate::cons_spec::ConsSpec;

#[derive(Debug)]
pub struct Spec<A> {
    pub(crate) actor_opt: Option<A>,
}

impl<A> Spec<A> {
    pub fn new(a: A) -> Self {
        Self { actor_opt: Some(a) }
    }

    pub fn and<T>(self, a: T) -> ConsSpec<T, Self> {
        ConsSpec {
            head: Spec::new(a),
            tail: self,
        }
    }
}
