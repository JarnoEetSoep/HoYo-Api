use std::error::Error;

#[derive(Debug)]
pub struct HoyoError(pub String);

impl Error for HoyoError {}
impl std::fmt::Display for HoyoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct CookieError(pub String);

impl Error for CookieError {}
impl std::fmt::Display for CookieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
