use std::fmt;

#[async_trait::async_trait]
pub trait Context: fmt::Debug + Send {}
