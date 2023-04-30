use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use machi::todo::Todos;

use std::{
    error::Error,
    fs::{self, File, ReadDir},
    io::{self, Stdout},
    path::PathBuf,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, StatefulWidget},
    Frame, Terminal,
};

mod todo;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout: Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend)?;

    // create app and run it
    let res: Result<(), io::Error> = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    // Deserialize json from .machi/*.json files

    let paths = fs::read_dir("./.machi").unwrap();

    let todos_list: Vec<Todos> = paths
        .map(|p| {
            let path: PathBuf = p.unwrap().path();
            let path_str: &str = path.to_str().unwrap();
            let todos: Todos =
                serde_json::from_str(&fs::read_to_string(path_str).unwrap()).unwrap();
            return todos;
        })
        .collect();

    let mut todos_selected: usize = 0;

    loop {
        let _ = terminal.draw(|f: &mut Frame<B>| ui(f, &todos_list, &todos_selected));
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Esc {
                return Ok(());
            }

            if key.code == KeyCode::Down {
                todos_selected = todos_selected + 1;
                if todos_selected >= todos_list.len() {
                    todos_selected = 0;
                }
            }

            if key.code == KeyCode::Up {
                if todos_selected == 0 {
                    todos_selected = todos_list.len() - 1;
                } else {
                    todos_selected = todos_selected - 1;
                }

            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, todos_list: &Vec<Todos>, todos_selected: &usize) {
    // Layouts
    let vl: Vec<Rect> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(4)])
        .split(f.size());

    let hl: Vec<Rect> = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(10)])
        .split(vl[0]);

    // Todos
    let todos_block: Block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::Blue))
        .title(" global ")
        .title_alignment(Alignment::Center);

    let todos = todos_list
        .iter()
        .map(|t| ListItem::new(t.name.clone()))
        .collect::<Vec<ListItem>>();

    //[ListItem::new(todos_json.name.clone())];

    let tds_list = List::new(todos).block(todos_block).highlight_style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
    );

    let mut todos_state = ListState::default();

    todos_state.select(Option::Some(*todos_selected));

    f.render_stateful_widget(tds_list, hl[0], &mut todos_state);

    // Todo

    let mut title: String = String::from(" ");
    title.push_str(&todos_list[*todos_selected].name);
    title.push_str(" ");

    let todo_block: Block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::Blue))
        .title(title)
        .title_alignment(Alignment::Center);

    let todo: Vec<ListItem> = todos_list[*todos_selected]
        .todo_list
        .iter()
        .map(|td| {
            let mut check: String = if td.done { "[x]" } else { "[ ]" }.to_owned();
            check.push_str(" ");
            check.push_str(&td.title);
            return ListItem::new(check);
        })
        .collect();

    let todo_list = List::new(todo).block(todo_block).highlight_style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Green),
    );

    let mut todo_state_selected = ListState::default();
    todo_state_selected.select(Option::Some(0));

    f.render_stateful_widget(todo_list, hl[1], &mut todo_state_selected);

    // Commands
    let commands: Block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .border_type(BorderType::Thick);
    f.render_widget(commands, vl[1]);
}
