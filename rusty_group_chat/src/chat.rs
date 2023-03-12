use serde::{Deserialize, Serialize};

use crate::User;

#[derive(Deserialize, Serialize)]
pub enum ChatSender {
    System,
    User(User),
}

#[derive(Deserialize, Serialize)]
pub struct Chat {
    pub from: ChatSender,
    pub message: String,
}

impl Chat {
    pub fn from_system(message: &str) -> Chat {
        Chat {
            from: ChatSender::System,
            message: message.to_owned(),
        }
    }

    pub fn from_user(message: &str, user: &User) -> Chat {
        Chat {
            from: ChatSender::User(user.clone()),
            message: message.to_owned(),
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_string(chat_as_str: &str) -> Chat {
        serde_json::from_str(&chat_as_str).unwrap()
    }
}
