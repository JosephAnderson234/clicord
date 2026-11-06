use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use protocol::{valid_message, valid_username, ChatMessageOut, Envelope};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{timeout, Duration};

use crate::state::AppState;

pub async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sink, mut stream) = socket.split();

    // First frame must be `Hello` — no auth yet, so we trust the username
    // the client sends. This is the seam where real auth replaces this step.
    let username = match timeout(Duration::from_secs(5), stream.next()).await {
        Ok(Some(Ok(Message::Text(text)))) => match serde_json::from_str::<Envelope>(&text) {
            Ok(Envelope::Hello(hello)) if valid_username(&hello.username) => hello.username,
            _ => return, // first frame wasn't a valid Hello, drop the connection
        },
        _ => return,
    };

    let mut rx = state.tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        loop {
            let msg = match rx.recv().await {
                Ok(msg) => msg,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => break,
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
            };
            let envelope = Envelope::ChatMessageOut(msg);
            let Ok(json) = serde_json::to_string(&envelope) else {
                continue;
            };
            if sink.send(Message::Text(json)).await.is_err() {
                break; // client disconnected
            }
        }
    });

    let tx = state.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = stream.next().await {
            let Ok(Envelope::ChatMessageIn(incoming)) = serde_json::from_str::<Envelope>(&text)
            else {
                continue;
            };
            if !valid_message(&incoming.text) {
                continue;
            }
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let out = ChatMessageOut {
                from: username.clone(),
                text: incoming.text,
                ts,
            };
            let _ = tx.send(out); // ignore error if no subscribers are listening
        }
    });

    // whichever task finishes first means the connection is done; cancel the other
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}
