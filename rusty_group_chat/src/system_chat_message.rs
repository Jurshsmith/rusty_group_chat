use crate::User;

pub struct SystemChatMessage;

impl SystemChatMessage {
    pub fn user_joined(user: &User) -> String {
        format!("{} joined.", user.name)
    }

    pub fn user_left(user: &User) -> String {
        format!("{} left.", user.name)
    }
}
