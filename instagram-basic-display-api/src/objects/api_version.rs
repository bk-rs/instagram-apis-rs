use std::fmt;

#[derive(Debug, Clone)]
pub enum ApiVersion {
    V12,
}
impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V12 => write!(f, "v12.0"),
        }
    }
}
