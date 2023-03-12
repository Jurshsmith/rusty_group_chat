use rusty_group_chat::UserRepo;

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

///////////////////////////////////
//      ServerWS BOUNDARY       //
/////////////////////////////////
use axum::extract::ws::{Message, WebSocket};
use futures::stream::{SplitSink, SplitStream};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast::{self, error::SendError, Receiver, Sender};

type Task = tokio::task::JoinHandle<()>;

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

    pub fn stream_into_client(
        &self,
        client_ws_sink: SplitSink<WebSocket, Message>,
    ) -> tokio::task::JoinHandle<()> {
        self.subscribe_and_get_stream().into_client(client_ws_sink)
    }

    pub fn subscribe_and_get_stream(&self) -> ServerWSStream {
        ServerWSStream {
            stream: self.channel.subscribe(),
        }
    }

    // TODO:  Ensure message string is serializable
    /// Stream from passed client stream to all connected clients' websockets
    pub fn stream_from_client_to_clients<T>(
        self,
        mut client_ws_stream: SplitStream<WebSocket>,
        payload_processor: T,
    ) -> tokio::task::JoinHandle<()>
    where
        T: Fn(&str) -> String + Send + 'static,
    {
        tokio::spawn(async move {
            // client_ws_stream will receive serializable payload
            while let Some(Ok(Message::Text(payload))) = client_ws_stream.next().await {
                println!("Streaming payload from client to Clients: {}", &payload);

                let message = payload_processor(&payload);
                let _send_result = self.clone().stream_to_clients(message);
            }
        })
    }

    //TODO:  Ensure message is serializable
    /// Broadcasts message to all connected client websockets
    pub fn stream_to_clients(&self, message: String) -> Result<usize, SendError<String>> {
        self.channel.send(message)
    }

    pub async fn cleanup_tasks(&self, mut send_task: Task, mut recv_task: Task) {
        // If any one of the tasks run to completion, we abort the other.

        tokio::select! {
            _result = (&mut send_task) => recv_task.abort(),
            _result = (&mut recv_task) => send_task.abort(),
        }
    }
}

///////////////////////////////////
//    ServerWSStream BOUNDARY   //
/////////////////////////////////

pub struct ServerWSStream {
    stream: Receiver<String>,
}

impl ServerWSStream {
    pub fn into_client(
        mut self,
        mut client_ws_sink: SplitSink<WebSocket, Message>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            while let Ok(payload) = self.stream.recv().await {
                println!("Streaming payload to client : {}", &payload);
                // TODO: Reject chat sent from current user
                if client_ws_sink.send(Message::Text(payload)).await.is_err() {
                    break;
                }
            }
        })
    }
}
