use crate::app::App;
use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    // TODO: Split the layout
    // let [area1, area2, area3 ...] =
    let mut area_list = frame.size();
    area_list.width = area_list.width / 8;

    let mut info_area = frame.size();
    info_area.width = info_area.width - area_list.width;
    info_area.x = area_list.width;

    let cities: Vec<String> = app.cities.clone();

    let list_component = List::new(cities)
        .block(Block::default().title("Cities").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        );

    let mut state = ListState::default();
    state.select(Some(app.index_selected));

    // TODO: render the list of cities
    frame.render_stateful_widget(list_component, area_list, &mut state);

    // TODO: Create the weather info component

    let weather_info_text = Paragraph::new(format!(
        "City: {}\nWeather: {}\nTemperature: {}°C",
        app.current_city_weather.name,
        app.current_city_weather.weather.0,
        app.current_city_weather.weather.1
    ))
    .block(Block::default().title("Weather Info").borders(Borders::ALL))
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::White));

    // TODO: Render the weather info component
    frame.render_widget(weather_info_text, info_area);
}
