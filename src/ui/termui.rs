use std::io::{ self, Stdout };
use std::time::Duration;

use anyhow::{ Context, Result };
use crossterm::{
    event::{ self, Event, KeyCode, KeyEventKind, EnableMouseCapture, DisableMouseCapture },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};
use ratatui::{ prelude::*, widgets::* };

pub struct AppUI<'a> {
    pub tab_names: Vec<&'a str>,
    pub index: usize,
}

impl<'a> AppUI<'a> {
    pub fn new() -> AppUI<'a> {
        AppUI {
            tab_names: vec!["Main Tab", "Memory", "World File", "Other..."],
            index: 0,
        }
    }

    pub fn next_tab(&mut self) {
        self.index = (self.index + 1) % self.tab_names.len();
    }

    pub fn previous_tab(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.tab_names.len() - 1;
        }
    }
}

// Run the application loop.
//
//  This is where you would handle events and update the application
// state. This example exits when the user presses 'q'. Other styles of application loops are
// possible, for example, you could have multiple application states and switch between them based
// on events, or you could have a single application state and update it based on events.
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: AppUI) -> Result<bool> {
    loop {
        terminal.draw(|frame| ui(frame, &app))?;

        // Check if the user has pressed 'q'.
        // This is where you would handle events. This example just
        // checks if the user has pressed 'q' and returns true if they have. It does not handle any other
        // events.
        // There is a 100ms timeout on the event poll so that the application can exit in a timely
        // manner, and to ensure that the terminal is rendered at least once every 100ms.
        if event::poll(Duration::from_millis(100)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(true);
                        }
                        KeyCode::Right | KeyCode::Char('l') => app.next_tab(),
                        KeyCode::Left | KeyCode::Char('h') => app.previous_tab(),
                        _ => {}
                    }
                }
            }
        }

        return Ok(false);
    }
}

pub fn ui(f: &mut Frame, app: &AppUI) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(size);

    let block = Block::default().on_white().black();
    f.render_widget(block, size);

    let titles = app.tab_names
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![first.yellow(), rest.green()])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().cyan().on_gray())
        .highlight_style(Style::default().bold().on_black());
    f.render_widget(tabs, chunks[0]);

    let inner = match app.index {
        0 => Block::default().title("Inner 0").borders(Borders::ALL),
        1 => Block::default().title("Inner 1").borders(Borders::ALL),
        2 => Block::default().title("Inner 2").borders(Borders::ALL),
        3 => Block::default().title("Inner 3").borders(Borders::ALL),
        _ => unreachable!(),
    };
    f.render_widget(inner, chunks[1]);
}

/// Setup the terminal.
///
/// This is where you would enable raw mode, enter the alternate screen, and
/// hide the cursor. This example does not handle errors. A more robust application would probably
/// want to handle errors and ensure that the terminal is restored to a sane state before exiting.
pub fn setup_termui() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).context(
        "unable to enter alternate screen"
    )?;
    return Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed");
}

/// Restore the terminal.
///
/// This is where you disable raw mode, leave the alternate screen, and show the cursor.
pub fn restore_terminal(termui: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(termui.backend_mut(), LeaveAlternateScreen, DisableMouseCapture).context(
        "unable to switch to main screen"
    )?;
    termui.show_cursor().context("unable to show cursor")
}

/// Render the application. This is where you would draw the application UI. This example just
/// draws a greeting.
pub fn render_frame(frame: &mut Frame) {
    let area = frame.size();

    frame.render_widget(Paragraph::new("TODO (press 'q' to quit)").white().on_blue(), area);
}
