use std::fmt;

#[derive(Debug, Clone)]
pub struct IncorrectLoginCredentials;
impl fmt::Display for IncorrectLoginCredentials {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid username or password")
    }
}
