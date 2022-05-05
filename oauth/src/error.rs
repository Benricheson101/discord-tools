use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknon scope {0}")]
    UnknownScope(String),
}
