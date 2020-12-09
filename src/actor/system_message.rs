#[derive(Debug)]
pub enum SystemMessage {
    Shutdown(Shutdown),
}

#[derive(Debug, Default)]
pub struct Shutdown {}
