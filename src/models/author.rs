use serde::{Deserialize, Serialize};
use crate::traits::PlainBytes;


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize, Serialize)]
pub struct Author {
    name: String,
    email: String,
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} <{}>", &self.name, &self.email)
    }
}
impl Author {
    pub fn new(name: &str, email: &str) -> Author {
        Author {
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn email(&self) -> String {
        self.email.to_string()
    }

    pub fn id(&self) -> u16 {
        let hash = crate::hash::keccak256(self.to_string().as_bytes());
        u16::from_le_bytes([hash[6], hash[1]])
    }
}

impl PlainBytes for Author {}
