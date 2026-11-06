use serde::{Deserialize, Serialize};

pub const MAX_USERNAME_BYTES: usize = 64;
pub const MAX_MESSAGE_BYTES: usize = 4096;

pub fn valid_username(username: &str) -> bool {
    !username.is_empty()
        && username.len() <= MAX_USERNAME_BYTES
        && username.trim() == username
        && !username.chars().any(char::is_control)
}

pub fn valid_message(text: &str) -> bool {
    !text.trim().is_empty()
        && text.len() <= MAX_MESSAGE_BYTES
        && !text.chars().any(char::is_control)
}

/// Envelope sent client -> server on connect. The name is validated for
/// length and control characters, but is not authenticated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hello {
    pub username: String,
}

/// Envelope sent client -> server for an outgoing chat message.
/// `from` and `ts` are NOT sent in each client message; the server fills
/// them in. The initial username is still unauthenticated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageIn {
    pub text: String,
}

/// Envelope sent server -> all clients (broadcast) for a chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageOut {
    pub from: String,
    pub text: String,
    pub ts: u64,
}

/// Top-level tagged union for everything that crosses the wire.
/// Using an internally-tagged enum so JSON looks like
/// `{ "type": "chat_message", "payload": { ... } }`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum Envelope {
    Hello(Hello),
    ChatMessageIn(ChatMessageIn),
    ChatMessageOut(ChatMessageOut),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_usernames() {
        assert!(!valid_username(""));
        assert!(!valid_username(" alice"));
        assert!(!valid_username("alice\nadmin"));
        assert!(!valid_username(&"a".repeat(MAX_USERNAME_BYTES + 1)));
        assert!(valid_username("alice"));
    }

    #[test]
    fn rejects_invalid_messages() {
        assert!(!valid_message("  "));
        assert!(!valid_message("hello\u{1b}[31m"));
        assert!(!valid_message(&"a".repeat(MAX_MESSAGE_BYTES + 1)));
        assert!(valid_message("hello"));
    }
}
