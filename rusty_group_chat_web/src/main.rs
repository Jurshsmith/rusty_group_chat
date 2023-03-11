mod app_server_config;
mod app_state;

use app_server_config::AppServerConfig;
use app_state::AppState;
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
use rusty_group_chat::{Chat, User, UserRepoError};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rusty_group_chat_web=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/group_chat", get(group_chat_handler))
        .with_state(app_state);

    let config = AppServerConfig::get();

    tracing::info!("Starting server at http://{}:{}/", config.host, config.port);

    tracing::info!(
        "Join Group Chat: http://{}:{}/group_chat",
        config.host,
        config.port
    );

    axum::Server::bind(&config.socket_address())
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
async fn group_chat_websocket(client_ws: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time. WS Connected to Client
    let (mut client_ws_sink, mut client_ws_stream) = client_ws.split();

    let mut current_user = User::empty();

    println!("INITIAL PART OF GROUP CHAT");

    // while let Some(Ok(message)) = client_ws_stream.next().await {
    //     dbg!("THIS IS THE MESSAGE: {}", &message);

    //     if let Message::Text(payload) = message {
    //         println!("THIS IS THE PAYLOAD: {}", &payload);

    //         let new_user = User::from(payload);

    //         match state.user_repo.add_user(new_user) {
    //             Ok(user) => {
    //                 // If user was added, quit the loop
    //                 current_user = user;
    //                 break;
    //             }
    //             // else we want to quit the whole function.
    //             Err(UserRepoError::UserAlreadyExists) => {
    //                 client_ws_sink
    //                     .send(Message::Text(String::from("Username already taken.")))
    //                     .await
    //                     .unwrap();

    //                 return;
    //             }
    //         }
    //     }
    // }

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut server_ws_stream = state.server_ws.subscribe_and_get_stream();

    // Now send the "joined" message to all subscribers.
    let message = format!("{} joined.", current_user.name);

    tracing::debug!("Broadcasting: {}", message);

    state.server_ws.broadcast_to_all_client_ws(message).unwrap();

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(payload) = server_ws_stream.recv().await {
            println!("THIS IS THE PAYLOAD SENT : {}", &payload);
            // In any websocket error, break loop.
            // TODO; Reject chat sent from current user
            if client_ws_sink.send(Message::Text(payload)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let server_ws = state.server_ws.clone();
    let current_user_name = current_user.name.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        // client_ws_stream will receive serializable payload
        while let Some(Ok(Message::Text(payload))) = client_ws_stream.next().await {
            println!("THIS IS THE PAYLOAD TO BROADCAST: {}", &payload);

            // Add username before message.
            let _send_result =
                server_ws.broadcast_to_all_client_ws(format!("{}: {}", current_user_name, payload));
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _result = (&mut send_task) => recv_task.abort(),
        _result = (&mut recv_task) => send_task.abort(),
    };

    // Send "user left" message (similar to "joined" above).
    state
        .server_ws
        .broadcast_to_all_client_ws(format!("{} left.", &current_user.name))
        .unwrap();

    // Remove username from map so new clients can take it again.
    state.user_repo.remove_user(current_user).unwrap();
}
