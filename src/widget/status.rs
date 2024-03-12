use ratatui::prelude::*;

pub struct Status<'f> {
    pub style: Style,
    pub left_spans: Vec<Span<'f>>,
    pub right_spans: Vec<Span<'f>>,
}

impl<'f> Status<'f> {
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<Style>,
    {
        self.style = style.into();
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn left_spans<I>(mut self, spans: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Span<'f>>,
    {
        self.left_spans = spans.into_iter().map(Into::into).collect();
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn right_spans<I>(mut self, spans: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Span<'f>>,
    {
        self.right_spans = spans.into_iter().map(Into::into).collect();
        self
    }
}

impl<'f> Widget for Status<'f> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);

        let layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        Line::default()
            .left_aligned()
            .spans(self.left_spans)
            .render(layout[0], buf);

        Line::default()
            .right_aligned()
            .spans(self.right_spans)
            .render(layout[1], buf);
    }
}

impl<'f> Default for Status<'f> {
    fn default() -> Self {
        let style = Style::default().fg(Color::DarkGray);

        Self {
            style,
            left_spans: Default::default(),
            right_spans: Default::default(),
        }
    }
}
