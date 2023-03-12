use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    // TODO: Add random color that can be used in CLI display
}
