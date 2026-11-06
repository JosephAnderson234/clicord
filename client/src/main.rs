mod app;
mod net;
mod ui;

use std::io::stdout;
use std::time::Duration;

use app::App;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use protocol::{valid_message, valid_username, ChatMessageOut, MAX_MESSAGE_BYTES};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

enum AppEvent {
    Input(Event),
    Message(ChatMessageOut),
    Disconnected,
    Tick,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // No auth yet: username is a plain prompt, replaced later by real login.
    println!("username:");
    let mut username = String::new();
    std::io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();
    if !valid_username(&username) {
        anyhow::bail!("el nombre debe tener entre 1 y 64 bytes, sin espacios alrededor ni caracteres de control");
    }

    let (out_tx, mut in_rx) = net::connect("ws://127.0.0.1:3000/ws", username.clone()).await?;

    enable_raw_mode()?;
    let mut stdout_handle = stdout();
    execute!(stdout_handle, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout_handle);
    let mut terminal = Terminal::new(backend)?;

    // crossterm::event::read() is blocking I/O, so it runs on its own OS
    // thread rather than inside a Tokio task, which would stall the executor.
    let (input_tx, mut input_rx) = mpsc::unbounded_channel::<Event>();
    std::thread::spawn(move || loop {
        if let Ok(ev) = event::read() {
            if input_tx.send(ev).is_err() {
                break;
            }
        }
    });

    let mut app = App::new(username);
    let mut disconnected = false;

    let tick_rate = Duration::from_millis(100);
    let mut interval = tokio::time::interval(tick_rate);
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    while !app.should_quit {
        terminal.draw(|f| ui::render(f, &app))?;

        let event = tokio::select! {
            Some(ev) = input_rx.recv() => AppEvent::Input(ev),
            msg = in_rx.recv() => match msg {
                Some(msg) => AppEvent::Message(msg),
                None => AppEvent::Disconnected,
            },
            _ = interval.tick() => AppEvent::Tick,
        };

        match event {
            // Windows' console backend reports both press and release for
            // every key; without this filter each keystroke is handled
            // twice (visually: every typed letter appears doubled).
            AppEvent::Input(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Esc => app.should_quit = true,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.should_quit = true;
                }
                KeyCode::Enter => {
                    if valid_message(&app.input) {
                        let text = std::mem::take(&mut app.input);
                        let _ = out_tx.send(text);
                    }
                }
                KeyCode::Char(c)
                    if !c.is_control() && app.input.len() + c.len_utf8() <= MAX_MESSAGE_BYTES =>
                {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                _ => {}
            },
            AppEvent::Input(_) => {}
            AppEvent::Message(msg) => app.push_message(msg),
            AppEvent::Disconnected => {
                app.should_quit = true;
                disconnected = true;
            }
            AppEvent::Tick => app.on_tick(),
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    if disconnected {
        anyhow::bail!("conexión cerrada por el servidor");
    }
    Ok(())
}
