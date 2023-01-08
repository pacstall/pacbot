pub mod github;
pub mod messages;
pub mod website;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type PacResult = Result<(), Error>;
