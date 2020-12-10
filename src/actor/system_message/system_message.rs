use super::*;

#[derive(Debug)]
pub enum SystemMessage {
    Shutdown(Shutdown),
    GetChildren(GetChildren),
}
