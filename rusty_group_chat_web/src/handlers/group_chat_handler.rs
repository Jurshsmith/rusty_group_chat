use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::IntoResponse,
};
use futures::stream::StreamExt;
use futures::SinkExt;
use rusty_group_chat::{Chat, SystemChatMessage, User};

use crate::app_state::AppState;

pub struct GroupChatHandler;

impl GroupChatHandler {
    pub async fn join(
        ws: WebSocketUpgrade,
        State(state): State<Arc<AppState>>,
        user: Query<User>,
    ) -> impl IntoResponse {
        ws.on_upgrade(|socket| Self::_join(socket, state, user.0))
    }

    async fn _join(client_ws: WebSocket, state: Arc<AppState>, current_user: User) {
        let (mut client_ws_sink, client_ws_stream) = client_ws.split();

        match state.user_repo.add_user(&current_user) {
            Ok(()) => {
                let send_task = state.server_ws.stream_into_client(client_ws_sink);

                // TODO: Return a SystemChatMessage struct instead
                let user_joined_msg = SystemChatMessage::user_joined(&current_user);
                state.server_ws.stream_to_clients(user_joined_msg).unwrap();

                let current_user_ = current_user.clone();

                let recv_task = state
                    .server_ws
                    .clone()
                    .stream_from_client_to_clients(client_ws_stream, move |message: &str| {
                        Chat::from_user(message, &current_user_).to_string()
                    });

                state.server_ws.cleanup_tasks(send_task, recv_task).await;

                // TODO: Return a SystemChatMessage struct instead
                let user_left_msg = SystemChatMessage::user_left(&current_user);
                state.server_ws.stream_to_clients(user_left_msg).unwrap();

                // Remove username from map so new clients can take it again.
                state.user_repo.remove_user(current_user).unwrap();
            }
            Err(user_repo_error) => {
                // TODO: Return a SystemChatMessage struct instead
                // Send error current client websocket
                client_ws_sink
                    .send(Message::Text(user_repo_error.into()))
                    .await
                    .unwrap();
            }
        }
    }
}
