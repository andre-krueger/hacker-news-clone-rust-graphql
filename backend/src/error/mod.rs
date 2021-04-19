use std::fmt;

#[derive(Debug, Clone)]
pub struct IncorrectLoginCredentials;
impl fmt::Display for IncorrectLoginCredentials {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid username or password")
    }
}

#[derive(Debug, Clone)]
pub struct UserNotFound;
impl fmt::Display for UserNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User not found")
    }
}

#[derive(Debug, Clone)]
pub struct Forbidden;
impl fmt::Display for Forbidden {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Forbidden")
    }
}
