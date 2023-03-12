use crate::{Chat, User};

pub struct SystemChatMessage;

impl SystemChatMessage {
    pub fn user_joined(user: &User) -> String {
        let message = format!("{} joined.", user.name);

        Chat::from_system(&message).to_string()
    }

    pub fn user_left(user: &User) -> String {
        let message = format!("{} left.", user.name);

        Chat::from_system(&message).to_string()
    }
}
