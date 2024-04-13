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
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    let event_handler = EventHandler::new(100);
    let mut tui = Tui::new(terminal, event_handler);

    tui.init()?;
    app.cities = connection::get_cities().await;
    app.add_weather(app.cities[0].clone()).await;

    // Start the main loop.
    while app.running {
        tui.draw(&mut app);

        // TODO: Handle events.
        if let Event::Key(key_event) = tui.events.next().await? {
            handle_key_events(key_event, &mut app).await?;
        }
    }

    // TODO: Reset the terminal if the app has been terminated
    tui.exit()?;
    Ok(())
}
