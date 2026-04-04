use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Text},
    widgets::{Block, List, ListItem, Paragraph},
};

pub struct App {
    selected_index: usize,
    factories: Vec<&'static str>,
}

impl App {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            factories: vec![
                "Iron Refinement Center",
                "Recycling Facility",
                "Fuel Power Plant",
            ],
        }
    }

    fn render_home(&self, _area: Rect) -> Paragraph<'static> {
        let ascii_art = Text::from(vec![
            Line::from(""),
            Line::from(""),
            Line::from(""),
            Line::from(""),
            Line::from(""),
            Line::from(""),
            Line::from(
                " ███████╗ █████╗ ████████╗██╗███████╗███████╗ █████╗  ██████╗████████╗ ██████╗ ██████╗ ██╗███████╗███████╗ ",
            ),
            Line::from(
                " ██╔════╝██╔══██╗╚══██╔══╝██║██╔════╝██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗██║██╔════╝██╔════╝ ",
            ),
            Line::from(
                " ███████╗███████║   ██║   ██║███████╗█████╗  ███████║██║        ██║   ██║   ██║██████╔╝██║█████╗  ███████╗ ",
            ),
            Line::from(
                " ╚════██║██╔══██║   ██║   ██║╚════██║██╔══╝  ██╔══██║██║        ██║   ██║   ██║██╔══██╗██║██╔══╝  ╚════██║ ",
            ),
            Line::from(
                " ███████║██║  ██║   ██║   ██║███████║██║     ██║  ██║╚██████╗   ██║   ╚██████╔╝██║  ██║██║███████╗███████║ ",
            ),
            Line::from(
                " ╚══════╝╚═╝  ╚═╝   ╚═╝   ╚═╝╚══════╝╚═╝     ╚═╝  ╚═╝ ╚═════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═╝╚══════╝╚══════╝ ",
            ),
            Line::from(""),
            Line::from(""),
            Line::from(""),
            Line::from(""),
            Line::from(""),
            Line::from(""),
        ]);

        Paragraph::new(ascii_art)
            .block(Block::new())
            .style(Style::new().cyan())
            .centered()
    }

    fn render_factory_list(&self, _area: Rect) -> List<'_> {
        let items: Vec<ListItem> = self
            .factories
            .iter()
            .enumerate()
            .map(|(i, factory)| {
                let style = if i == self.selected_index {
                    Style::new().yellow().bold()
                } else {
                    Style::new().white()
                };
                ListItem::new(*factory).style(style)
            })
            .collect();

        List::new(items)
            .block(
                Block::bordered()
                    .title(" Factories ")
                    .border_style(Style::new().blue()),
            )
            .highlight_style(Style::new().on_blue().yellow())
    }

    fn render(&self, frame: &mut Frame) {
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
                .areas(frame.area());

        let list = self.render_factory_list(left);
        frame.render_widget(list, left);

        let home = self.render_home(right);
        frame.render_widget(home, right);
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut terminal = ratatui::init();
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.selected_index = self
                            .selected_index
                            .saturating_add(1)
                            .min(self.factories.len() - 1);
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.selected_index = self.selected_index.saturating_sub(1);
                    }
                    KeyCode::Enter => {
                        // TODO: Handle selection
                    }
                    _ => {}
                }
            }
        }
        ratatui::restore();
        Ok(())
    }
}

pub fn run_tui() {
    let mut app = App::new();
    if let Err(e) = app.run() {
        eprintln!("Error running TUI: {e}");
    }
}
