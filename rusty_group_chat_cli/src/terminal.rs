pub struct Terminal;

impl Terminal {
    pub fn read_line() -> Result<String, io::Error> {
        CrosstermTerminal::read_line()
    }

    pub fn clear_all() {
        CrosstermTerminal::clear_all();
    }

    pub fn clear_last_line() {
        CrosstermTerminal::clear_n_previous_lines(1);
    }
}

///////////////////////////////////
//      CROSSTERM BOUNDARY      //
/////////////////////////////////
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    terminal, ExecutableCommand,
};
use std::io::{self, stdout};

struct CrosstermTerminal;

// Helps Abstract CrossTerm functions to provide an higher level generic API
impl CrosstermTerminal {
    pub fn read_line() -> Result<String, io::Error> {
        let mut line = String::new();
        while let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Char(c) => {
                    line.push(c);
                }
                _ => {}
            }
        }

        Ok(line)
    }

    pub fn clear_all() {
        let mut stdout = stdout();

        stdout.execute(cursor::RestorePosition).unwrap();
        stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
            .unwrap();
    }

    pub fn clear_n_previous_lines(line: u16) {
        let mut stdout = stdout();

        stdout.execute(cursor::MoveToPreviousLine(line)).unwrap();
        stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorDown))
            .unwrap();
    }
}
