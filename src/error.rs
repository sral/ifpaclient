#[derive(thiserror::Error, Debug)]
pub enum IfpaError {
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },
}
