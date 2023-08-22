use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::IntoResponse,
};
use futures::{
    sink::SinkExt,
    stream::{SplitStream, StreamExt},
};
use rusty_group_chat::{Chat, SystemChatMessage, User};

use crate::app_state::AppState;

pub async fn join(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    user: Query<User>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| _join(socket, state, user.0))
}

async fn _join(my_socket: WebSocket, state: Arc<AppState>, current_user: User) {
    let (mut me, mut receiver) = my_socket.split();

    // Allow user create a channel
    // Allow users join different channels
    match state.user_repo.add_user(&current_user) {
        Ok(()) => {
            let mut group_channel_subscription = state.group_chat.channel.subscribe();

            let user_joined_msg = SystemChatMessage::user_joined(&current_user);
            state.group_chat.channel.send(user_joined_msg).unwrap();

            let send_task = tokio::spawn(async move {
                while let Ok(msg) = group_channel_subscription.recv().await {
                    if me.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            });

            let group_channel = Arc::clone(&state.group_chat.channel);

            let receive_task = tokio::spawn({
                let current_user = current_user.clone();

                async move {
                    while let Some(Ok(Message::Text(msg))) = receiver.next().await {
                        group_channel
                            .send(Chat::from_user(&msg, &current_user).serialize())
                            .unwrap();
                    }
                }
            });

            cleanup_tasks_when_either_completes(send_task, receive_task).await;

            disconnect_user(&current_user, &state).await;
        }
        Err(user_repo_error) => {
            me.send(Message::Text(user_repo_error.into()))
                .await
                .unwrap();
        }
    }
}

async fn _get_username(current_user_stream: &mut SplitStream<WebSocket>) -> String {
    current_user_stream
        .next()
        .await
        .expect("Username required")
        .unwrap()
        .to_text()
        .unwrap()
        .to_string()
}

async fn disconnect_user(user: &User, state: &AppState) {
    let user_left_msg = SystemChatMessage::user_left(&user);
    state.group_chat.channel.send(user_left_msg).unwrap();
    state.user_repo.remove_user(user.clone());
}

type Task = tokio::task::JoinHandle<()>;

async fn cleanup_tasks_when_either_completes(mut send_task: Task, mut recv_task: Task) {
    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _result = (&mut send_task) => recv_task.abort(),
        _result = (&mut recv_task) => send_task.abort(),
    }
}
