

#[derive(Debug)]
pub enum MergeError {
    Conflict(String),
    InvalidObject,
    Other(String),
}
