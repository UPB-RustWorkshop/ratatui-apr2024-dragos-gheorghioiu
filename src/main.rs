use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui_templates::app::{App, AppResult};
use ratatui_templates::connection;
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    // TODO:  the terminal user interface
    let event_handler = EventHandler::new(100);
    let mut tui = Tui::new(terminal, event_handler);

    // TODO: init the terminal
    tui.init()?;
    app.cities = connection::get_cities().await;
    app.add_weather(app.cities[0].clone()).await;

    // Start the main loop.
    while app.running {
        // TODO: Render the user interface.
        tui.draw(&mut app);

        // TODO: Handle events.
        match tui.events.next().await? {
            Event::Key(key_event) => {
                handle_key_events(key_event, &mut app).await?;
            }
            _ => {}
        }
    }

    // TODO: Reset the terminal if the app has been terminated
    tui.exit()?;
    Ok(())
}
