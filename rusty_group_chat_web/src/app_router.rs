use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{app_state::AppState, handlers::GroupChatHandler};

pub struct AppRouter {
    pub router: Router,
}

impl AppRouter {
    pub fn new() -> Self {
        let app_state = Arc::new(AppState::new());

        AppRouter {
            router: Router::new()
                .route("/", get(GroupChatHandler::join))
                .with_state(app_state),
        }
    }
}
