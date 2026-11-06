mod state;
mod ws;

use axum::{
    extract::{State, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use state::AppState;

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.max_message_size(protocol::MAX_MESSAGE_BYTES + 512)
        .max_frame_size(protocol::MAX_MESSAGE_BYTES + 512)
        .on_upgrade(move |socket| ws::handle_socket(socket, state))
}

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind to 127.0.0.1:3000");
    println!("clicord server listening on ws://127.0.0.1:3000/ws");
    axum::serve(listener, app).await.expect("server error");
}
