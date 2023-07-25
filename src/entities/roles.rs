use std::fmt;

#[derive(Debug)]
pub enum Roles {
    Admin,
    Author,
}

impl fmt::Display for Roles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
