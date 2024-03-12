use ratatui::style::Color;

pub struct Theme {
    pub background: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Rgb(40, 45, 60),
        }
    }
}
