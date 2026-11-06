use protocol::ChatMessageOut;
use tokio::sync::broadcast;

/// One global room for the MVP. When channels/rooms are added, this becomes
/// something like `HashMap<ChannelId, broadcast::Sender<ChatMessageOut>>`
/// behind a RwLock — the subscribe/send pattern in `ws.rs` does not change.
#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<ChatMessageOut>,
}

impl AppState {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(1024);
        Self { tx }
    }
}
