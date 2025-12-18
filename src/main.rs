use std::io;
use tuiznos::App;

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let mut app = App {
        context: String::new(),
        terminal,
    };
    let result = app.run();
    ratatui::restore();
    result
}
