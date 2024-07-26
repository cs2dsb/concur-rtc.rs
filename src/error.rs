use std::fmt::Debug;

use gloo::utils::errors::JsError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JsError: {0}")]
    JsError(#[from] JsError),

    /// Invalid configuration provided to [`crate::SocketBuilder`]
    ///
    /// These errors are only returned from the bulder and are all fatal
    #[error("InvalidConfig: {0}")]
    InvalidConfig(String),
}
