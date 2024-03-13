use ratatui::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Result, Write};
use std::path::PathBuf;
use tui_textarea::TextArea;

pub struct View<'v> {
    pub path: PathBuf,
    pub textarea: TextArea<'v>,
    pub modified: bool,
}

impl<'v> View<'v> {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut textarea = if let Ok(metadata) = path.metadata() {
            if metadata.is_file() {
                let mut textarea = BufReader::new(File::open(&path)?)
                    .lines()
                    .collect::<Result<TextArea>>()?;

                if textarea.lines().iter().any(|line| line.starts_with('\t')) {
                    textarea.set_hard_tab_indent(true);
                }

                textarea
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("{:?} is not a file", path),
                ));
            }
        } else {
            TextArea::default()
        };

        textarea.set_line_number_style(Style::default().fg(Color::DarkGray));
        textarea.set_cursor_line_style(Style::default());

        Ok(Self {
            textarea,
            path,
            modified: false,
        })
    }

    #[inline]
    pub fn cursor(&self) -> (usize, usize) {
        self.textarea.cursor()
    }

    pub fn save(&mut self) -> Result<()> {
        if !self.modified {
            return Ok(());
        }

        let mut file = BufWriter::new(File::create(&self.path)?);

        for line in self.textarea.lines() {
            file.write_all(line.as_bytes())?;
            file.write_all(b"\n")?;
        }

        self.modified = false;

        Ok(())
    }

    pub fn widget(&'v self) -> impl Widget + 'v {
        Renderer::new(self)
    }
}

impl<'v> Widget for View<'v> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.textarea.widget().render(area, buf)
    }
}

impl<'v> Renderer<'v> {
    pub fn new(editor: &'v View<'v>) -> Self {
        Self(editor)
    }
}

pub struct Renderer<'r>(&'r View<'r>);

impl<'r> Widget for Renderer<'r> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.0.textarea.widget().render(area, buf);
    }
}
