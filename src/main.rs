mod audioplayback;

use anyhow::Result;
use crossterm::{
  event::{self, Event::Key, KeyCode::Char},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
  prelude::*,
  widgets::{Block, Borders, Paragraph},
};


fn startup() -> Result<()> {
  enable_raw_mode()?;
  execute!(std::io::stderr(), EnterAlternateScreen)?;
  Ok(())
}

fn shutdown() -> Result<()> {
  execute!(std::io::stderr(), LeaveAlternateScreen)?;
  disable_raw_mode()?;
  Ok(())
}

// App state
struct App {
  should_quit: bool,
}

// App ui render function
fn ui(app: &App, f: &mut Frame) {
    let layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(15),
        Constraint::Percentage(85)
    ])
    .split(f.size());

    let right_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(70),
        Constraint::Percentage(30)
    ])
    .split(layout[1]);

  f.render_widget(Paragraph::new("Toolbar").block(Block::new().borders(Borders::all())), layout[0]);
  f.render_widget(Paragraph::new("Main").block(Block::new().borders(Borders::all())), right_layout[0]);
  f.render_widget(Paragraph::new("niggatons").block(Block::new().borders(Borders::all())), right_layout[1])
}

// App update function
fn update(app: &mut App) -> Result<()> {
  if event::poll(std::time::Duration::from_millis(250))? {
    if let Key(key) = event::read()? {
      if key.kind == event::KeyEventKind::Press {
        match key.code {
                    Char('p') => audioplayback::play(),
                    Char('q') => Ok(app.should_quit = true),
                    _ => {},
                }
                //TODO ILLI LOGIC BARIYIRI
            }
        }
    }
  Ok(())
}

fn run() -> Result<()> {
  // ratatui terminal
  let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

  // application state
  let mut app = App { counter: 0, should_quit: false };

  loop {
    // application update
    update(&mut app)?;

    // application render
    t.draw(|f| {
      ui(&app, f);
    })?;

    // application exit
    if app.should_quit {
      break;
    }
  }

  Ok(())
}

fn main() -> Result<()> {
  // setup terminal
  startup()?;

  let result = run();

  // teardown terminal before unwrapping Result of app run
  shutdown()?;

  result?;

  Ok(())
}