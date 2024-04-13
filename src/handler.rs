use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('q') => {
            app.running = false;
        }
        KeyCode::Down => {
            app.index_selected = (app.index_selected + 1) % app.cities.len();
            app.add_weather(app.cities[app.index_selected].clone())
                .await;
        }
        KeyCode::Up => {
            if app.index_selected > 0 {
                app.index_selected -= 1;
            } else {
                app.index_selected = app.cities.len() - 1;
            }
            app.add_weather(app.cities[app.index_selected].clone())
                .await;
        }
        _ => {}
    }
    Ok(())
}
