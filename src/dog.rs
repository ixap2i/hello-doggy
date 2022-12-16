use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};
use std::error;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dog {
    pub message: String,
    pub status: String,
}

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "json response has error")
    }
}

impl error::Error for Dog {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}