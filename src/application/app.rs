use super::super::TxtArea;
use super::super::{Mode, Transition, Vim};
use crossterm::event::DisableMouseCapture;
use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use ratatui::DefaultTerminal;
use ratatui::layout::{Constraint, Direction, Layout};
use std::io;

pub struct App {
    pub context: String,
    pub terminal: DefaultTerminal, // type alias for Terminal<CrosstermBackend<Stdout>>
}

impl App {
    pub fn run(&mut self) -> io::Result<()> {
        let mut textarea = TxtArea::new()?;
        textarea.set_normal_block(Mode::Normal);
        textarea.set_normal_cursor_style(Mode::Normal);
        let mut vim = Vim::new(Mode::Normal);

        loop {
            self.terminal.draw(|f| {
                f.render_widget(textarea.textarea(), f.area());
            })?;

            vim = match vim.transition(crossterm::event::read()?.into(), textarea.textarea_mut()) {
                Transition::Mode(mode) if vim.mode != mode => {
                    textarea.set_normal_block(mode);
                    textarea.set_normal_cursor_style(mode);
                    Vim::new(mode)
                }
                Transition::Nop | Transition::Mode(_) => vim,
                Transition::Pending(input) => vim.with_pending(input),
                Transition::Quit => break,
            }
        }

        self.cleanup(&mut textarea)?;
        Ok(())
    }

    //TODO: This still needs some work and testing
    fn cleanup(&mut self, txtarea: &mut TxtArea) -> io::Result<()> {
        disable_raw_mode()?;
        crossterm::execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;

        //NOTE: Saving this to a file using textarea.lines()
        //      Need this for my input box
        println!("Lines: {:?}", txtarea.textarea.lines());

        Ok(())
    }
}
