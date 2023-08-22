use rusty_group_chat::UserRepo;

// Our shared state
pub struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    pub user_repo: UserRepo,
    // Channel used to send messages to all connected clients.
    // Also used to receive messages from all connected clients ?
    pub group_chat: GroupChat,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            user_repo: UserRepo::new(),
            group_chat: GroupChat::new(),
        }
    }
}

use std::sync::Arc;
use tokio::sync::broadcast::{self, Sender};

#[derive(Clone)]
pub struct GroupChat {
    pub channel: Arc<Sender<String>>,
}

impl GroupChat {
    pub fn new() -> Self {
        let (sink_channel, _stream_channel) = broadcast::channel(100);

        GroupChat {
            channel: Arc::new(sink_channel),
        }
    }
}
