mod server_config;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use rusty_group_chat::{Chat, User, UserRepo, UserRepoError};
use server_config::ServerConfig;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// Our shared state
struct AppState {
    // We require unique usernames. This tracks which usernames have been taken.
    user_repo: UserRepo,
    // Channel used to send messages to all connected clients.
    channel: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rusty_group_chat_web=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Set up application state for use with with_state().
    let (channel, _channel_for_receiving) = broadcast::channel(100);

    let app_state = Arc::new(AppState {
        user_repo: UserRepo::new(),
        channel,
    });

    let app = Router::new()
        .route("/group_chat", get(group_chat_handler))
        .with_state(app_state);

    let config = ServerConfig::get();

    tracing::info!("Starting server at http://{}:{}/", config.host, config.port);

    axum::Server::bind(&config.host_with_port().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn group_chat_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| group_chat_websocket(socket, state))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn group_chat_websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    let mut current_user = User::empty();

    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(payload) = message {
            let new_user = User::from(payload);

            match state.user_repo.add_user(new_user) {
                Ok(user) => {
                    // If user was added, quit the loop
                    current_user = user;
                    break;
                }
                // else we want to quit the whole function.
                Err(UserRepoError::UserAlreadyExists) => {
                    sender
                        .send(Message::Text(String::from("Username already taken.")))
                        .await
                        .unwrap();
                }
            }
        }
    }

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut channel = state.channel.subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = format!("{} joined.", current_user.name);

    tracing::debug!("Broadcasting: {}", msg);

    let _send_result = state.channel.send(msg);

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(payload) = channel.recv().await {
            let chat = Chat::from(payload);
            // In any websocket error, break loop.
            if sender.send(Message::Text(chat.into())).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let channel = state.channel.clone();
    let current_user_name = current_user.name.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(payload))) = receiver.next().await {
            let chat = Chat::from(payload);

            // Add username before message.
            let _send_result = channel.send(format!("{}: {}", current_user_name, chat.message));
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _result = (&mut send_task) => recv_task.abort(),
        _result = (&mut recv_task) => send_task.abort(),
    };

    // Send "user left" message (similar to "joined" above).
    let msg = format!("{} left.", &current_user.name);

    tracing::debug!("{}", msg);

    let _send_result = state.channel.send(msg);

    // Remove username from map so new clients can take it again.
    state.user_repo.remove_user(current_user).unwrap();
}
