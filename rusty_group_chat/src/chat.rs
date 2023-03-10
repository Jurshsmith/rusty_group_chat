use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Chat {
    pub message: String,
}

impl Into<String> for Chat {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl From<String> for Chat {
    fn from(chat: String) -> Self {
        serde_json::from_str(&chat).unwrap()
    }
}
