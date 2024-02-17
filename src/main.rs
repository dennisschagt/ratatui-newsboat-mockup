use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{self, Constraint, Layout},
    prelude::{CrosstermBackend, Terminal},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{List, ListState},
    Frame,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut feed_list = FeedListFormAction::new();

    loop {
        terminal.draw(|f| feed_list.draw(f))?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Up {
                    if let Some(index) = feed_list.list_state.selected() {
                        if index > 0 {
                            feed_list.list_state.select(Some(index - 1))
                        }
                    };
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Down {
                    if let Some(index) = feed_list.list_state.selected() {
                        feed_list.list_state.select(Some(index + 1))
                    };
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

struct FeedListFormAction {
    list_state: ListState,
}

impl FeedListFormAction {
    fn new() -> Self {
        Self {
            list_state: ListState::default().with_selected(Some(1))
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
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

        let list = List::new([
                Line::default().spans([Span::raw("   1     (0/37) "), Span::styled("Newsboat", Style::default().light_red())]),
                Line::styled("   2 N  (2/902) xkcd.com", Style::default().bold()),
                Line::raw("   3     (0/11) Software Defeined Radio with HackRF"),
                Line::raw("   4     (0/23) Debiania"),
                Line::raw("   5    (0/190) Ctrl blog"),
            ])
            .highlight_style(Style::default().yellow().on_blue().bold());

        frame.render_stateful_widget(
            list,
            layout[1],
            &mut self.list_state,
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
            .flat_map(|(key, action)| {
                [key.yellow().bold(), ":".white(), action.gray(), " ".white()].into_iter()
            })
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
