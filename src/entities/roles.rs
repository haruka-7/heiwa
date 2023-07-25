use std::fmt;

#[derive(Debug)]
pub enum Roles {
    ADMIN,
    AUTHOR,
}

impl fmt::Display for Roles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}