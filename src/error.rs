use std::error::Error;
use std::fmt;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub struct AppError {
    message: String,
    inner: Option<Box<dyn Error + Send + Sync>>,
}

impl AppError {
    pub fn new(message: &str) -> Self {
        AppError {
            message: message.to_owned(),
            inner: None,
        }
    }

    pub fn with_error<E: Error + Send + Sync + 'static>(message: &str, err: E) -> Self {
        AppError {
            message: message.to_owned(),
            inner: Some(err.into()),
        }
    }
}

impl Default for AppError {
    fn default() -> AppError {
        AppError {
            message: String::from("Unknown Error"), 
            inner: None,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(err) = &self.inner {
            write!(f, "{}: {}", self.message, err)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(err) = &self.inner {
            Some(err.as_ref())
        } else {
            None
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::with_error("I/O Error", e)
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(e: toml::ser::Error) -> Self {
        Self::with_error("Invalid TOML format Error", e)
    }
}
impl From<toml::de::Error> for AppError {
    fn from(e: toml::de::Error) -> Self {
        Self::with_error("Invalid TOML format Error", e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        Self::with_error("HTTP request Error", e)
    }
}

impl From<serde_json::error::Error> for AppError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::with_error("JSON format Error", e)
    }
}

impl From<std::net::AddrParseError> for AppError {
    fn from(e: std::net::AddrParseError) -> Self {
        Self::with_error("IP address format Error", e)
    }
}

