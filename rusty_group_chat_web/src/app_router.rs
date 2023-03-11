use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{app_state::AppState, handlers::GroupChatHandler};

pub struct AppRouter;

impl AppRouter {
    pub fn new() -> Router {
        let app_state = Arc::new(AppState::new());

        Router::new()
            .route("/group_chat", get(GroupChatHandler::join))
            .with_state(app_state)
    }
}
