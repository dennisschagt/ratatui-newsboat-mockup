use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{self, Constraint, Layout},
    prelude::{CrosstermBackend, Terminal},
    style::{Style, Stylize},
    text::Line,
    widgets::Paragraph,
    Frame,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let feed_list = FeedListFormAction::new();

    loop {
        terminal.draw(|f| feed_list.draw(f))?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

struct FeedListFormAction {}

impl FeedListFormAction {
    fn new() -> Self {
        Self {}
    }

    fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(frame.size());

        let title_style = Style::new().yellow().on_blue().bold();
        frame.render_widget(
            Line::styled("Newsboat r2.xx - Your feeds", title_style),
            layout[0],
        );

        frame.render_widget(
            Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_black(),
            layout[1],
        );

        let bindings = vec![
            ("q", "Quit"),
            ("ENTER", "Open"),
            ("n", "Next unread"),
            ("r", "Reload"),
            ("R", "Reload All"),
            ("A", "Mark Read"),
            ("C", "Mark All Read"),
            ("?", "Search"),
            ("h", "Help"),
        ];

        let bindings: Vec<_> = bindings
            .iter()
            .flat_map(|(key, action)| [
                    key.yellow().bold(),
                    ":".white(),
                    action.gray(),
                    " ".white(),
                ].into_iter())
            .collect();

        frame.render_widget(
            Line::default()
                .style(Style::default().on_blue())
                .spans(bindings),
            layout[2],
        );

        frame.render_widget(Line::raw(":"), layout[3]);
    }
}
