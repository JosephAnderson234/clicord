use protocol::ChatMessageOut;

pub struct App {
    pub username: String,
    pub input: String,
    pub messages: Vec<ChatMessageOut>,
    pub tick_count: u64,
    pub should_quit: bool,
}

impl App {
    pub fn new(username: String) -> Self {
        Self {
            username,
            input: String::new(),
            messages: Vec::new(),
            tick_count: 0,
            should_quit: false,
        }
    }

    pub fn push_message(&mut self, msg: ChatMessageOut) {
        self.messages.push(msg);
    }

    pub fn on_tick(&mut self) {
        // Placeholder for animated state (spinner frame, cursor blink, etc.)
        // driven at a fixed cadence rather than on every render.
        self.tick_count = self.tick_count.wrapping_add(1);
    }
}
