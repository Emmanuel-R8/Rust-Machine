use std::io::{ self, Stdout };
use std::time::Duration;

use anyhow::{ Context, Result };
use crossterm::{
    event::{ self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};
use ratatui::{ prelude::*, widgets::* };

// Contains all the state data for the UI
pub struct AppUI<'a> {
    pub tab_index: usize,
    pub tab_names: Vec<&'a str>,

    pub table_state: TableState,
    pub table_content: Vec<Vec<&'a str>>,
}

impl<'a> AppUI<'a> {
    pub fn new() -> AppUI<'a> {
        AppUI {
            //
            // Global App statae
            tab_index: 0,
            tab_names: vec!["Main Tab", "Table example", "Memory", "World File", "Other..."],
            //
            // Tab: Table example
            table_state: TableState::default(),
            table_content: vec![
                vec!["Row11", "Row12", "Row13"],
                vec!["Row21", "Row22", "Row23"],
                vec!["Row31", "Row32", "Row33"],
                vec!["Row41", "Row42", "Row43"],
                vec!["Row51", "Row52", "Row53"],
                vec!["Row61", "Row62\nTest", "Row63"],
                vec!["Row71", "Row72", "Row73"],
                vec!["Row81", "Row82", "Row83"],
                vec!["Row91", "Row92", "Row93"]
            ],
        }
    }

    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tab_names.len();
    }

    pub fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.tab_names.len() - 1;
        }
    }

    //
    // Tab example with a table content
    //
    pub fn next_cell(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => (i + 1) % self.table_content.len(),
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous_cell(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => (i - 1) % self.table_content.len(),
            None => 0,
        };
        self.table_state.select(Some(i));
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
        .select(app.tab_index)
        .style(Style::default().cyan().on_gray())
        .highlight_style(Style::default().bold().on_black());
    f.render_widget(tabs, chunks[0]);

    let inner = match app.tab_index {
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
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
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
