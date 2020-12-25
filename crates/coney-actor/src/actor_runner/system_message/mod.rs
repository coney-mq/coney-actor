mod shutdown;
pub use shutdown::Shutdown;
pub use shutdown::ShutdownReason;

#[derive(Debug)]
pub enum SystemMessage {
    Shutdown(Shutdown),
}
