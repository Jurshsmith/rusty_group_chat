use std::sync::Arc;

use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::IntoResponse,
};
use futures::stream::StreamExt;
use rusty_group_chat::{SystemChatMessage, User};

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
        let (client_ws_sink, client_ws_stream) = client_ws.split();

        let send_task = state.server_ws.stream_into_client(client_ws_sink);

        // TODO: Return a SystemChatMessage struct instead
        let user_joined_msg = SystemChatMessage::user_joined(&current_user);
        state.server_ws.stream_to_clients(user_joined_msg).unwrap();

        let current_user_name = current_user.name.clone();

        let recv_task = state.server_ws.clone().stream_from_client_to_clients(
            client_ws_stream,
            move |message: &str| {
                // TODO: Pattern Match if it's a system chat message or a user chat message and preprocess accordingly
                format!("{}: {}", current_user_name, message)
            },
        );

        state.server_ws.cleanup_tasks(send_task, recv_task).await;

        // TODO: Return a SystemChatMessage struct instead
        let user_left_msg = SystemChatMessage::user_left(&current_user);
        state.server_ws.stream_to_clients(user_left_msg).unwrap();

        // Remove username from map so new clients can take it again.
        state.user_repo.remove_user(current_user).unwrap();
    }
}
