use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    // TODO: Add random color that can be used in CLI display
}

impl From<String> for User {
    fn from(user: String) -> Self {
        serde_json::from_str(&user).unwrap()
    }
}

impl User {
    pub fn empty() -> Self {
        User {
            name: "".to_string(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
}
