pub mod cons_spec;
pub mod get_spec;
pub mod spec;

pub mod prelude {
    pub use crate::cons_spec::ConsSpec;
    pub use crate::spec::Spec;
}

#[cfg(test)]
mod tests;
