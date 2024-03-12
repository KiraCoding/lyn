use crate::theme::Theme;
use crate::widget::{Status, View};
use crossterm::event::{read, DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::Widget;
use ratatui::prelude::*;
use std::io::{stdout, Result, Stdout};
use std::path::PathBuf;
use tui_textarea::{Input, Key};

pub struct App<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    theme: Theme,
    view: View<'a>,
}

impl<'a> App<'a> {
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        let mut stdout = stdout();

        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let terminal = Terminal::new(CrosstermBackend::new(stdout))?;
        let theme = Theme::default();
        let view = View::new(path.into())?;

        Ok(Self {
            terminal,
            theme,
            view,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.terminal.draw(|frame| {
                let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)])
                    .split(frame.size());

                frame.render_widget(self.view.widget(), layout[0]);

                let (line, row) = self.view.cursor();
                let lr = format!("{}:{} ", line + 1, row + 1);

                let modified = if self.view.modified { "[+]" } else { "" };
                let path = format!(" ~/{}{}", self.view.path.display(), modified);

                Status::default()
                    .left_spans(vec![path])
                    .right_spans(vec![lr])
                    .style(
                        Style::default()
                            .fg(Color::DarkGray)
                            .bg(self.theme.background),
                    )
                    .render(layout[1], frame.buffer_mut());
            })?;

            match read()?.into() {
                Input { key: Key::Esc, .. } => break,
                Input {
                    key: Key::Char('s'),
                    ctrl: true,
                    ..
                } => self.view.save()?,
                input => {
                    self.view.modified = self.view.textarea.input(input);
                }
            };
        }

        Ok(())
    }
}

impl<'a> Drop for App<'a> {
    fn drop(&mut self) {
        self.terminal.show_cursor().unwrap();
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap()
    }
}
