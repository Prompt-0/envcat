pub mod app;
pub mod ui;

use app::App;
use crate::types::EnvEntry;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

pub fn run_tui(entries: Vec<EnvEntry>) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(entries);
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("TUI Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if app.is_searching {
                    match key.code {
                        KeyCode::Enter | KeyCode::Esc => {
                            app.is_searching = false;
                        }
                        KeyCode::Char(c) => {
                            app.search_query.push(c);
                            app.apply_filter();
                        }
                        KeyCode::Backspace => {
                            app.search_query.pop();
                            app.apply_filter();
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('c') => {
                            if key.modifiers.contains(KeyModifiers::CONTROL) {
                                return Ok(());
                            } else if let Some(e) = app.selected_entry() {
                                app.status_message = Some(format!("Selected: {}={}", e.key, e.masked_value));
                            }
                        }
                        KeyCode::Char('/') | KeyCode::Char('f') => {
                            app.is_searching = true;
                        }
                        KeyCode::Char(' ') => {
                            app.toggle_global_reveal();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.next();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.previous();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
