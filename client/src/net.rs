use futures_util::{SinkExt, StreamExt};
use protocol::{ChatMessageIn, ChatMessageOut, Envelope, Hello};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

/// Connects to the server, sends the initial `Hello`, and spawns the two
/// tasks that bridge the WebSocket to plain channels the UI loop can
/// `select!` on without knowing anything about tungstenite or serde.
///
/// Returns:
/// - a sender for outgoing chat text (what the user typed)
/// - a receiver for incoming chat messages (broadcast from the server)
pub async fn connect(
    url: &str,
    username: String,
) -> anyhow::Result<(
    mpsc::UnboundedSender<String>,
    mpsc::UnboundedReceiver<ChatMessageOut>,
)> {
    let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    let hello = Envelope::Hello(Hello { username });
    write
        .send(Message::Text(serde_json::to_string(&hello)?))
        .await?;

    let (out_tx, mut out_rx) = mpsc::unbounded_channel::<String>();
    let (in_tx, in_rx) = mpsc::unbounded_channel::<ChatMessageOut>();

    // Outgoing: text typed by the user -> ChatMessageIn envelope -> socket
    tokio::spawn(async move {
        while let Some(text) = out_rx.recv().await {
            let envelope = Envelope::ChatMessageIn(ChatMessageIn { text });
            let Ok(json) = serde_json::to_string(&envelope) else {
                continue;
            };
            if write.send(Message::Text(json)).await.is_err() {
                break; // connection dropped
            }
        }
    });

    // Incoming: socket -> ChatMessageOut envelope -> channel the UI reads
    tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = read.next().await {
            if let Ok(Envelope::ChatMessageOut(msg)) = serde_json::from_str::<Envelope>(&text) {
                if in_tx.send(msg).is_err() {
                    break; // UI loop went away
                }
            }
        }
    });

    Ok((out_tx, in_rx))
}
