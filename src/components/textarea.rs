use super::super::Mode;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use tui_textarea::TextArea;

#[derive(Clone, Debug)]
pub struct TxtArea {
    pub textarea: TextArea<'static>,
}

impl TxtArea {
    pub fn new() -> io::Result<Self> {
        let textarea = match env::args().nth(1) {
            Some(path) => {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;

                TextArea::new(lines)
            }
            None => TextArea::default(),
        };

        Ok(Self { textarea })
    }

    pub fn set_normal_block(&mut self, mode: Mode) {
        self.textarea.set_block(mode.block());
    }

    pub fn set_normal_cursor_style(&mut self, mode: Mode) {
        self.textarea.set_cursor_style(mode.cursor_style());
    }

    pub fn textarea_mut(&mut self) -> &mut TextArea<'static> {
        &mut self.textarea
    }

    pub fn textarea(&self) -> &TextArea<'static> {
        &self.textarea
    }
}
