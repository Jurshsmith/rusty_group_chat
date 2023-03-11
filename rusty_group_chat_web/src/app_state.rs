use rusty_group_chat::UserRepo;
use tokio::sync::broadcast::{self, error::SendError, Receiver, Sender};

// Our shared state
pub struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    pub user_repo: UserRepo,
    // Channel used to send messages to all connected clients.
    // Also used to receive messages from all connected clients ?
    pub server_ws: ServerWS,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            user_repo: UserRepo::new(),
            server_ws: ServerWS::new(),
        }
    }
}

#[derive(Clone)]
pub struct ServerWS {
    channel: Box<Sender<String>>,
}

impl ServerWS {
    fn new() -> Self {
        // Sink Channel is for sending message. Send messages into a Sink Channel
        // Stream Channel is for receiving messages. Receive message from an Stream Channel
        // But since the data first enters the sink, and we want to get the data as early as possible
        // We are fine with just using the sink_channel as the channel for all websocket
        let (sink_channel, _stream_channel) = broadcast::channel(100);

        ServerWS {
            channel: Box::new(sink_channel),
        }
    }

    pub fn subscribe_and_get_stream(&self) -> Receiver<String> {
        self.channel.subscribe()
    }

    /// Broadcasts message to all connected client websockets
    pub fn broadcast_to_all_client_ws(&self, message: String) -> Result<usize, SendError<String>> {
        self.channel.send(message)
    }
}
