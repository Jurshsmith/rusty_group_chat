use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    // TODO: Add random color that can be used in CLI display
}

impl User {
    pub fn from_name(name: String) -> User {
        User { name }
    }

    pub fn is_equal_to(&self, user: &User) -> bool {
        self.name == user.name
    }
}
