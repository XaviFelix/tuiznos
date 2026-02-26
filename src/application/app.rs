use super::super::{Mode, Transition, Vim};
use super::super::{OutputBox, TxtArea};
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

        let output_box = OutputBox::new(String::from("Gemini AI"));

        loop {
            self.terminal.draw(|f| {
                //NOTE: This sets up the layout
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(f.area());

                f.render_widget(&output_box.para, chunks[0]);
                f.render_widget(textarea.textarea(), chunks[1]);
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

    //TODO: Test this more
    fn cleanup(&mut self, txtarea: &mut TxtArea) -> io::Result<()> {
        disable_raw_mode()?;
        crossterm::execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;

        //TODO: Pass this to the output_box, also create a separate function for the Key::Enter
        //      match where this piece of code is needed
        println!("Lines: {:?}", txtarea.textarea.lines());

        Ok(())
    }
}
