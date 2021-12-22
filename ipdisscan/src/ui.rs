use crate::beacons::BeaconAnswer;
use color_eyre::eyre::Report;
use crossbeam::channel::Receiver;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io::{self, Stdout};
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap};
use tui::Terminal;

type ConcreteTerminal = Terminal<CrosstermBackend<Stdout>>;

const HELP: &str = "q: close, j/↓: next, k/↑: previous";

pub fn run(channel_receiving_end: Receiver<Vec<BeaconAnswer>>) -> Result<(), Report> {
    let mut terminal = init_terminal()?;
    let mut app = App::default();
    app.next();
    loop {
        app.update_answers(channel_receiving_end.clone())?;
        draw_frame(&mut terminal, &mut app)?;
        if event::poll(Duration::from_secs(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') | KeyCode::Down => app.next(),
                    KeyCode::Char('k') | KeyCode::Up => app.prev(),
                    _ => continue,
                };
            };
        }
    }
    cleanup_terminal(terminal)?;
    Ok(())
}

/// App holds the state of the application
#[derive(Debug, Clone, Default)]
struct App {
    server_answers: Vec<BeaconAnswer>,
    list_state: ListState,
}

impl App {
    fn get_cursor(&self) -> Option<usize> {
        self.list_state.selected()
        // self.list_state.selected().unwrap_or_default()
    }

    fn next(&mut self) {
        if self.server_answers.is_empty() {
            return;
        };
        let len = self.server_answers.len();
        let index = match self.get_cursor() {
            None => 0,
            Some(i) => match i {
                i if i >= len => 0, // loop
                _ => i + 1,
            },
        };
        self.list_state.select(Some(index));
    }

    fn prev(&mut self) {
        if self.server_answers.is_empty() {
            return;
        };
        let len = self.server_answers.len();
        let index = match self.get_cursor() {
            None => 0,
            Some(i) => match i {
                0 => len - 1, // loop
                _ => i - 1,
            },
        };
        self.list_state.select(Some(index));
    }

    fn get_info_text(&self) -> String {
        let index = match self.get_cursor() {
            None => return String::default(),
            Some(i) => i,
        };
        let info_text = match self.server_answers.get(index) {
            None => String::default(),
            Some(a) => a.payload.pretty_format(),
        };
        info_text
    }

    fn get_list_items(&self) -> Vec<ListItem> {
        self.server_answers
            .iter()
            .map(|a| ListItem::new(a.addr.to_string()))
            .collect()
    }

    fn update_answers(
        &mut self,
        channel_receiving_end: Receiver<Vec<BeaconAnswer>>,
    ) -> Result<(), Report> {
        loop {
            // drain the channel, only last element counts
            self.server_answers = match channel_receiving_end.try_recv() {
                Ok(a) => a,
                _ => break,
            };
        }
        Ok(())
    }
}

fn init_terminal() -> Result<ConcreteTerminal, Report> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    // terminal.clear()?;
    Ok(terminal)
}

fn cleanup_terminal(mut terminal: ConcreteTerminal) -> Result<(), Report> {
    disable_raw_mode()?;
    // terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn draw_frame(terminal: &mut ConcreteTerminal, app: &mut App) -> Result<(), Report> {
    terminal.draw(|f| {
        let app_clone = app.clone();
        let address_list = app_clone.get_list_items();
        let info_text = app.get_info_text();

        // Surrounding block
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" ipdisscan - ({}) ", HELP))
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        f.render_widget(block, f.size());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        // Top two main blocks
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(chunks[0]);

        // IP list block
        let list = List::new(address_list)
            .block(
                Block::default()
                    .title("Found devices")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        f.render_stateful_widget(list, main_chunks[0], &mut app.list_state);

        // Info block
        let info = Paragraph::new(info_text)
            .block(Block::default().title("Informations").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false });
        f.render_widget(info, main_chunks[1]);
    })?;
    Ok(())
}
