use ratatui::{
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap},
};

pub struct OutputBox {
    pub para: Paragraph<'static>,
}

impl OutputBox {
    pub fn new(title: String) -> Self {
        Self {
            para: Self::with_styled_borders(
                Paragraph::new(title.white()).wrap(Wrap { trim: true }),
            ),
        }
    }

    //TODO: Pass this to the render_widget(widget, area) function in run()
    //      the argument will be a new instance of a paragraph:
    //      Paragraph::new("Random string")
    fn with_styled_borders<'a>(para: Paragraph<'a>) -> Paragraph<'a> {
        para.block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("OutputBox Title"),
            // .border_style(Style::new().blue().on_white().bold().italic()),
        )
    }

    //TODO: THis will be dynamic in nature
    // fn placeholder_paragraph() -> Paragraph<'static> {
    //     let text = "This is placeholder text."; Paragraph::new(text.white()).wrap(Wrap { trim: true })
    // }
}

/*
 *
 use ratatui::text::Line;
 use ratatui::widgets::{Paragraph, Wrap};

let lines: Vec<Line> = messages
    .iter()
    .flat_map(|msg| msg.lines())
    .map(Line::from)
    .collect();


//NOTE: trim: false preserves indentation
let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
 *
 * */
