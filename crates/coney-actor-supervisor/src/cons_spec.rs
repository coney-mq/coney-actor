use crate::spec::Spec;

pub struct ConsSpec<Head, Tail> {
    pub(crate) head: Spec<Head>,
    pub(crate) tail: Tail,
}

impl<Head, Tail> ConsSpec<Head, Tail> {
    pub fn and<T>(self, a: T) -> ConsSpec<T, Self> {
        ConsSpec {
            head: Spec::new(a),
            tail: self,
        }
    }
}
